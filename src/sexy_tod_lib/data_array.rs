// [TRANSLATION_NOTE]: DataArray.h -> Rust unsafe 实现
// C++ 模板 DataArray 使用 placement new 和显式析构，Rust 中需要用 unsafe + 裸指针模拟

const DATA_ARRAY_INDEX_MASK: u32 = 65535;
const DATA_ARRAY_KEY_MASK: u32 = !65535;
const DATA_ARRAY_KEY_SHIFT: u32 = 16;
const DATA_ARRAY_MAX_SIZE: u32 = 65536;

#[repr(C)]
pub struct DataArrayItem<T> {
    pub m_item: T,
    pub m_id: u32,
}

pub struct DataArray<T> {
    pub m_block: *mut DataArrayItem<T>,
    pub m_max_used_count: u32,
    pub m_max_size: u32,
    pub m_free_list_head: u32,
    pub m_size: u32,
    pub m_next_key: u32,
    pub m_name: *const u8,
}

unsafe impl<T: Send> Send for DataArray<T> {}
unsafe impl<T: Sync> Sync for DataArray<T> {}

impl<T: Default> DataArray<T> {
    pub fn new() -> Self {
        DataArray {
            m_block: std::ptr::null_mut(),
            m_max_used_count: 0,
            m_max_size: 0,
            m_free_list_head: 0,
            m_size: 0,
            m_next_key: 1,
            m_name: std::ptr::null(),
        }
    }

    pub fn data_array_initialize(&mut self, the_max_size: u32, the_name: &str) {
        assert!(self.m_block.is_null());
        let size = std::mem::size_of::<DataArrayItem<T>>() * the_max_size as usize;
        let layout = std::alloc::Layout::from_size_align(size, std::mem::align_of::<DataArrayItem<T>>()).unwrap();
        self.m_block = unsafe { std::alloc::alloc(layout) as *mut DataArrayItem<T> };
        self.m_max_size = the_max_size;
        self.m_next_key = 1001;
        self.m_name = the_name.as_ptr();
    }

    pub unsafe fn data_array_dispose(&mut self) {
        if !self.m_block.is_null() {
            self.data_array_free_all();
            unsafe {
                let size = std::mem::size_of::<DataArrayItem<T>>() * self.m_max_size as usize;
                let layout = std::alloc::Layout::from_size_align(size, std::mem::align_of::<DataArrayItem<T>>()).unwrap();
                std::alloc::dealloc(self.m_block as *mut u8, layout);
            }
            self.m_block = std::ptr::null_mut();
            self.m_max_used_count = 0;
            self.m_max_size = 0;
            self.m_free_list_head = 0;
            self.m_size = 0;
            self.m_name = std::ptr::null();
        }
    }

    pub unsafe fn data_array_alloc(&mut self) -> *mut T {
        assert!(self.m_size < self.m_max_size, "Data array full");
        assert!(self.m_free_list_head <= self.m_max_used_count, "DataArrayAlloc error");
        let a_next: u32;
        if self.m_free_list_head == self.m_max_used_count {
            a_next = self.m_max_used_count;
            self.m_free_list_head = self.m_max_used_count + 1;
            self.m_max_used_count += 1;
        } else {
            a_next = self.m_free_list_head;
            self.m_free_list_head = (*self.m_block.add(self.m_free_list_head as usize)).m_id;
        }
        let a_new_item = &mut *self.m_block.add(a_next as usize);
        std::ptr::write_bytes(a_new_item, 0, 1);
        a_new_item.m_id = (self.m_next_key << DATA_ARRAY_KEY_SHIFT) | a_next;
        self.m_next_key += 1;
        if self.m_next_key == DATA_ARRAY_MAX_SIZE {
            self.m_next_key = 1;
        }
        self.m_size += 1;

        // Placement new: write default value
        std::ptr::write(&mut a_new_item.m_item, T::default());
        &mut a_new_item.m_item as *mut T
    }

    pub unsafe fn data_array_free(&mut self, the_item: *mut T) {
        let a_item = the_item as *mut DataArrayItem<T>;
        let an_id = (*a_item).m_id & DATA_ARRAY_INDEX_MASK;
        // Destructor not needed for Rust Default types
        (*a_item).m_id = self.m_free_list_head;
        self.m_free_list_head = an_id;
        self.m_size -= 1;
    }

    pub unsafe fn data_array_free_all(&mut self) {
        let mut a_item: *mut T = std::ptr::null_mut();
        while self.iterate_next(&mut a_item) {
            self.data_array_free(a_item);
        }
        self.m_free_list_head = 0;
        self.m_max_used_count = 0;
    }

    pub unsafe fn data_array_get_id(&self, the_item: *mut T) -> u32 {
        let a_item = the_item as *mut DataArrayItem<T>;
        (*a_item).m_id
    }

    pub unsafe fn iterate_next(&self, the_item: &mut *mut T) -> bool {
        let mut a_item = if the_item.is_null() {
            self.m_block
        } else {
            (*the_item as *mut DataArrayItem<T>).add(1)
        };
        let a_last = self.m_block.add(self.m_max_used_count as usize);
        while a_item < a_last {
            if (*a_item).m_id & DATA_ARRAY_KEY_MASK != 0 {
                *the_item = &mut (*a_item).m_item as *mut T;
                return true;
            }
            a_item = a_item.add(1);
        }
        false
    }

    pub unsafe fn data_array_try_to_get(&self, the_id: u32) -> *mut T {
        if the_id == 0 || (the_id & DATA_ARRAY_INDEX_MASK) >= self.m_max_size {
            return std::ptr::null_mut();
        }
        let a_block = &mut *self.m_block.add((the_id & DATA_ARRAY_INDEX_MASK) as usize);
        if a_block.m_id == the_id {
            &mut a_block.m_item as *mut T
        } else {
            std::ptr::null_mut()
        }
    }

    pub unsafe fn data_array_get(&self, the_id: u32) -> *mut T {
        let result = self.data_array_try_to_get(the_id);
        assert!(!result.is_null(), "DataArrayGet failed");
        result
    }
}

impl<T: Default> Default for DataArray<T> {
    fn default() -> Self {
        DataArray::new()
    }
}
