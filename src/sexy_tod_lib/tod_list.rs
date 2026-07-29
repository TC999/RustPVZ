// [TRANSLATION_NOTE]: TodList.h + TodList.cpp -> Rust unsafe 实现
// 自定义内存分配器 (TodAllocator) 和双向链表 (TodList<T>)
// 使用 unsafe 裸指针模拟 C++ 的 placement new / 显式内存管理

use crate::sexy_tod_lib::tod_debug::{tod_malloc, tod_free, _tod_assert};

pub const MAX_GLOBAL_ALLOCATORS: i32 = 128;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TodAllocator {
    pub m_free_list: *mut u8,
    pub m_block_list: *mut u8,
    pub m_grow_count: i32,
    pub m_total_items: i32,
    pub m_item_size: i32,
}

impl TodAllocator {
    pub fn new() -> Self {
        TodAllocator {
            m_free_list: std::ptr::null_mut(),
            m_block_list: std::ptr::null_mut(),
            m_grow_count: 0,
            m_total_items: 0,
            m_item_size: 0,
        }
    }

    pub fn initialize(&mut self, the_grow_count: i32, the_item_size: i32) {
        _tod_assert(the_item_size as usize >= std::mem::size_of::<*mut u8>(), file!(), line!(), "");
        self.m_free_list = std::ptr::null_mut();
        self.m_block_list = std::ptr::null_mut();
        self.m_grow_count = the_grow_count;
        self.m_total_items = 0;
        self.m_item_size = the_item_size;
    }

    pub fn dispose(&mut self) {
        self.free_all();
    }

    fn grow(&mut self) {
        _tod_assert(self.m_grow_count > 0, file!(), line!(), "");
        _tod_assert(self.m_item_size as usize >= std::mem::size_of::<*mut u8>(), file!(), line!(), "");

        let _block_size = (self.m_grow_count * self.m_item_size + std::mem::size_of::<*mut u8>() as i32) as usize;
        let a_block = tod_malloc((self.m_grow_count * self.m_item_size + std::mem::size_of::<*mut u8>() as i32) as i32);
        unsafe {
            *(a_block as *mut *mut u8) = self.m_block_list;
        }
        self.m_block_list = a_block;

        let mut a_free_list = self.m_free_list;
        let mut a_item = unsafe { a_block.add(std::mem::size_of::<*mut u8>()) };
        for _ in 0..self.m_grow_count {
            unsafe {
                *(a_item as *mut *mut u8) = a_free_list;
            }
            a_free_list = a_item;
            a_item = unsafe { a_item.add(self.m_item_size as usize) };
        }
        self.m_free_list = a_free_list;
    }

    pub fn is_pointer_from_allocator(&self, the_item: *mut u8) -> bool {
        let a_block_size = (self.m_grow_count * self.m_item_size) as usize;
        let mut a_ptr = self.m_block_list;
        while !a_ptr.is_null() {
            let a_item_ptr = the_item as usize;
            let a_block_ptr = unsafe { a_ptr.add(std::mem::size_of::<*mut u8>()) } as usize;
            if a_item_ptr >= a_block_ptr 
                && a_item_ptr < a_block_ptr + a_block_size 
                && (a_item_ptr - a_block_ptr) % self.m_item_size as usize == 0 
            {
                return true;
            }
            unsafe {
                a_ptr = *(a_ptr as *mut *mut u8);
            }
        }
        false
    }

    pub fn is_pointer_on_free_list(&self, the_item: *mut u8) -> bool {
        let mut a_ptr = self.m_free_list;
        while !a_ptr.is_null() {
            if the_item == a_ptr {
                return true;
            }
            unsafe {
                a_ptr = *(a_ptr as *mut *mut u8);
            }
        }
        false
    }

    pub fn alloc(&mut self, _the_item_size: i32) -> *mut u8 {
        self.m_total_items += 1;
        if self.m_free_list.is_null() {
            self.grow();
        }

        let an_item = self.m_free_list;
        unsafe {
            self.m_free_list = *(an_item as *mut *mut u8);
        }
        an_item
    }

    pub fn calloc(&mut self, the_item_size: i32) -> *mut u8 {
        let an_item = self.alloc(the_item_size);
        if !an_item.is_null() {
            unsafe {
                std::ptr::write_bytes(an_item, 0, the_item_size as usize);
            }
        }
        an_item
    }

    pub fn free(&mut self, the_item: *mut u8, _the_item_size: i32) {
        self.m_total_items -= 1;
        _tod_assert(self.is_pointer_from_allocator(the_item), file!(), line!(), "");
        _tod_assert(!self.is_pointer_on_free_list(the_item), file!(), line!(), "");
        unsafe {
            *(the_item as *mut *mut u8) = self.m_free_list;
        }
        self.m_free_list = the_item;
    }

    pub fn free_all(&mut self) {
        let mut a_block = self.m_block_list;
        while !a_block.is_null() {
            unsafe {
                let a_next = *(a_block as *mut *mut u8);
                tod_free(a_block);
                a_block = a_next;
            }
        }
        self.m_block_list = std::ptr::null_mut();
        self.m_free_list = std::ptr::null_mut();
        self.m_total_items = 0;
    }
}

// 全局分配器变量
static mut G_NUM_GLOBAL_ALLOCATORS: i32 = 0;
static mut G_GLOBAL_ALLOCATORS: [TodAllocator; MAX_GLOBAL_ALLOCATORS as usize] = [TodAllocator {
    m_free_list: std::ptr::null_mut(),
    m_block_list: std::ptr::null_mut(),
    m_grow_count: 0,
    m_total_items: 0,
    m_item_size: 0,
}; MAX_GLOBAL_ALLOCATORS as usize];

pub fn find_global_allocator(the_size: i32) -> *mut TodAllocator {
    unsafe {
        for i in 0..G_NUM_GLOBAL_ALLOCATORS {
            if G_GLOBAL_ALLOCATORS[i as usize].m_item_size == the_size {
                return &mut G_GLOBAL_ALLOCATORS[i as usize];
            }
        }

        _tod_assert(G_NUM_GLOBAL_ALLOCATORS < MAX_GLOBAL_ALLOCATORS - 1, file!(), line!(), "");

        let p_allocator = &mut G_GLOBAL_ALLOCATORS[G_NUM_GLOBAL_ALLOCATORS as usize];
        G_NUM_GLOBAL_ALLOCATORS += 1;
        p_allocator.initialize(16, the_size);
        p_allocator
    }
}

pub fn free_global_allocators() {
    unsafe {
        for i in 0..G_NUM_GLOBAL_ALLOCATORS {
            G_GLOBAL_ALLOCATORS[i as usize].free_all();
        }
        G_NUM_GLOBAL_ALLOCATORS = 0;
    }
}

// TodListNode - 双向链表节点
#[repr(C)]
pub struct TodListNode<T> {
    pub m_value: T,
    pub m_next: *mut TodListNode<T>,
    pub m_prev: *mut TodListNode<T>,
}

// TodList - 双向链表
pub struct TodList<T> {
    pub m_head: *mut TodListNode<T>,
    pub m_tail: *mut TodListNode<T>,
    pub m_size: i32,
    pub mp_allocator: *mut TodAllocator,
}

impl<T: Copy> TodList<T> {
    pub fn new() -> Self {
        TodList {
            m_head: std::ptr::null_mut(),
            m_tail: std::ptr::null_mut(),
            m_size: 0,
            mp_allocator: std::ptr::null_mut(),
        }
    }

    pub fn get_head(&self) -> *mut TodListNode<T> {
        _tod_assert(!self.m_head.is_null(), file!(), line!(), "");
        self.m_head
    }

    pub fn get_tail(&self) -> *mut TodListNode<T> {
        _tod_assert(!self.m_tail.is_null(), file!(), line!(), "");
        self.m_tail
    }

    pub fn add_head(&mut self, the_head: T) {
        if self.mp_allocator.is_null() {
            self.mp_allocator = find_global_allocator(std::mem::size_of::<TodListNode<T>>() as i32);
        }

        unsafe {
            let a_node = (*self.mp_allocator).calloc(std::mem::size_of::<TodListNode<T>>() as i32) as *mut TodListNode<T>;
            if !a_node.is_null() {
                (*a_node).m_value = the_head;
            }
            (*a_node).m_next = self.m_head;
            (*a_node).m_prev = std::ptr::null_mut();
            if !self.m_head.is_null() {
                (*self.m_head).m_prev = a_node;
            } else {
                self.m_tail = a_node;
            }
            self.m_size += 1;
            self.m_head = a_node;
        }
    }

    pub fn add_tail(&mut self, the_tail: T) {
        if self.mp_allocator.is_null() {
            self.mp_allocator = find_global_allocator(std::mem::size_of::<TodListNode<T>>() as i32);
        }

        unsafe {
            let a_node = (*self.mp_allocator).calloc(std::mem::size_of::<TodListNode<T>>() as i32) as *mut TodListNode<T>;
            if !a_node.is_null() {
                (*a_node).m_value = the_tail;
            }
            (*a_node).m_next = std::ptr::null_mut();
            (*a_node).m_prev = self.m_tail;
            if !self.m_tail.is_null() {
                (*self.m_tail).m_next = a_node;
            } else {
                self.m_head = a_node;
            }
            self.m_size += 1;
            self.m_tail = a_node;
        }
    }

    pub fn remove_head(&mut self) -> T {
        unsafe {
            let a_head = self.m_head;
            let a_sec_node = (*a_head).m_next;
            self.m_head = a_sec_node;
            if !a_sec_node.is_null() {
                (*a_sec_node).m_prev = std::ptr::null_mut();
            } else {
                self.m_tail = std::ptr::null_mut();
            }

            let a_val = (*a_head).m_value;
            self.m_size -= 1;
            (*self.mp_allocator).free(a_head as *mut u8, std::mem::size_of::<TodListNode<T>>() as i32);
            a_val
        }
    }

    pub fn remove_at(&mut self, the_node: *mut TodListNode<T>) -> *mut TodListNode<T> {
        unsafe {
            let a_next = (*the_node).m_next;
            if !(*the_node).m_prev.is_null() {
                (*(*the_node).m_prev).m_next = a_next;
            } else {
                self.m_head = a_next;
            }

            if !a_next.is_null() {
                (*a_next).m_prev = (*the_node).m_prev;
            } else {
                self.m_tail = (*the_node).m_prev;
            }

            self.m_size -= 1;
            (*self.mp_allocator).free(the_node as *mut u8, std::mem::size_of::<TodListNode<T>>() as i32);
            a_next
        }
    }

    pub fn find(&self, the_item: &T) -> *mut TodListNode<T>
    where T: PartialEq
    {
        let mut a_node = self.m_head;
        while !a_node.is_null() {
            unsafe {
                if (*a_node).m_value == *the_item {
                    return a_node;
                }
                a_node = (*a_node).m_next;
            }
        }
        std::ptr::null_mut()
    }

    pub fn remove_all(&mut self) {
        let mut a_node = self.m_head;
        while !a_node.is_null() {
            unsafe {
                let temp = a_node;
                a_node = (*a_node).m_next;
                (*self.mp_allocator).free(temp as *mut u8, std::mem::size_of::<TodListNode<T>>() as i32);
            }
        }

        self.m_size = 0;
        self.m_head = std::ptr::null_mut();
        self.m_tail = std::ptr::null_mut();
    }

    pub fn set_allocator(&mut self, the_allocator: *mut TodAllocator) {
        _tod_assert(self.m_size == 0, file!(), line!(), "");
        self.mp_allocator = the_allocator;
    }

    pub fn is_empty(&self) -> bool {
        self.m_size == 0
    }

    pub fn size(&self) -> i32 {
        self.m_size
    }
}

impl<T> Drop for TodList<T> {
    fn drop(&mut self) {
        let mut a_node = self.m_head;
        while !a_node.is_null() {
            unsafe {
                let temp = a_node;
                a_node = (*a_node).m_next;
                if !self.mp_allocator.is_null() {
                    (*self.mp_allocator).free(temp as *mut u8, std::mem::size_of::<TodListNode<T>>() as i32);
                }
            }
        }
    }
}
