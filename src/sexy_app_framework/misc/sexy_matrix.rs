// [TRANSLATION_NOTE]: SexyMatrix.h -> Rust struct
// C++ 联合体(union)映射为 Rust 结构体，SexyTransform2D 继承映射为组合

use super::sexy_vector::{SexyVector2, SexyVector3};

#[derive(Clone, Copy, Debug)]
pub struct SexyMatrix3 {
    pub m: [[f32; 3]; 3],
}

impl SexyMatrix3 {
    pub fn new() -> Self {
        SexyMatrix3 { m: [[0.0; 3]; 3] }
    }

    pub fn zero_matrix(&mut self) {
        self.m = [[0.0; 3]; 3];
    }

    pub fn load_identity(&mut self) {
        self.m = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    }

    pub fn mul_vec2(&self, the_vec: &SexyVector2) -> SexyVector2 {
        SexyVector2::new_xy(
            self.m[0][0] * the_vec.x + self.m[0][1] * the_vec.y + self.m[0][2],
            self.m[1][0] * the_vec.x + self.m[1][1] * the_vec.y + self.m[1][2],
        )
    }

    pub fn mul_vec3(&self, the_vec: &SexyVector3) -> SexyVector3 {
        SexyVector3::new_xyz(
            self.m[0][0] * the_vec.x + self.m[0][1] * the_vec.y + self.m[0][2] * the_vec.z,
            self.m[1][0] * the_vec.x + self.m[1][1] * the_vec.y + self.m[1][2] * the_vec.z,
            self.m[2][0] * the_vec.x + self.m[2][1] * the_vec.y + self.m[2][2] * the_vec.z,
        )
    }

    pub fn mul_matrix(&self, the_mat: &SexyMatrix3) -> SexyMatrix3 {
        let mut result = SexyMatrix3::new();
        for i in 0..3 {
            for j in 0..3 {
                result.m[i][j] = self.m[i][0] * the_mat.m[0][j]
                    + self.m[i][1] * the_mat.m[1][j]
                    + self.m[i][2] * the_mat.m[2][j];
            }
        }
        result
    }

    pub fn mul_assign_matrix(&mut self, the_mat: &SexyMatrix3) {
        *self = self.mul_matrix(the_mat);
    }
}

impl Default for SexyMatrix3 {
    fn default() -> Self {
        let mut m = SexyMatrix3::new();
        m.load_identity();
        m
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SexyTransform2D {
    pub matrix: SexyMatrix3,
}

impl SexyTransform2D {
    pub fn new() -> Self {
        let mut m = SexyMatrix3::new();
        m.load_identity();
        SexyTransform2D { matrix: m }
    }

    pub fn new_with_identity(_load_identity: bool) -> Self {
        let mut m = SexyMatrix3::new();
        if _load_identity {
            m.load_identity();
        }
        SexyTransform2D { matrix: m }
    }

    pub fn new_from_matrix(the_matrix: &SexyMatrix3) -> Self {
        SexyTransform2D {
            matrix: *the_matrix,
        }
    }

    pub fn assign(&mut self, the_mat: &SexyMatrix3) {
        self.matrix = *the_mat;
    }

    pub fn translate(&mut self, tx: f32, ty: f32) {
        self.matrix.m[0][2] += tx;
        self.matrix.m[1][2] += ty;
    }

    pub fn rotate_rad(&mut self, rot: f32) {
        let cos_r = rot.cos();
        let sin_r = rot.sin();
        let mut rot_mat = SexyMatrix3::new();
        rot_mat.load_identity();
        rot_mat.m[0][0] = cos_r;
        rot_mat.m[0][1] = -sin_r;
        rot_mat.m[1][0] = sin_r;
        rot_mat.m[1][1] = cos_r;
        self.matrix = self.matrix.mul_matrix(&rot_mat);
    }

    pub fn rotate_deg(&mut self, rot: f32) {
        self.rotate_rad(rot * std::f32::consts::PI / 180.0);
    }

    pub fn scale(&mut self, sx: f32, sy: f32) {
        self.matrix.m[0][0] *= sx;
        self.matrix.m[1][1] *= sy;
    }
}

impl Default for SexyTransform2D {
    fn default() -> Self {
        SexyTransform2D::new()
    }
}

#[derive(Clone, Debug)]
pub struct Transform {
    pub m_complex: bool,
    pub m_have_rot: bool,
    pub m_have_scale: bool,
    pub m_trans_x1: f32,
    pub m_trans_y1: f32,
    pub m_trans_x2: f32,
    pub m_trans_y2: f32,
    pub m_scale_x: f32,
    pub m_scale_y: f32,
    pub m_rot: f32,
    pub m_matrix: SexyTransform2D,
    m_need_calc_matrix: bool,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            m_complex: false,
            m_have_rot: false,
            m_have_scale: false,
            m_trans_x1: 0.0,
            m_trans_y1: 0.0,
            m_trans_x2: 0.0,
            m_trans_y2: 0.0,
            m_scale_x: 1.0,
            m_scale_y: 1.0,
            m_rot: 0.0,
            m_matrix: SexyTransform2D::new(),
            m_need_calc_matrix: false,
        }
    }

    pub fn reset(&mut self) {
        self.m_complex = false;
        self.m_have_rot = false;
        self.m_have_scale = false;
        self.m_trans_x1 = 0.0;
        self.m_trans_y1 = 0.0;
        self.m_trans_x2 = 0.0;
        self.m_trans_y2 = 0.0;
        self.m_scale_x = 1.0;
        self.m_scale_y = 1.0;
        self.m_rot = 0.0;
        self.m_need_calc_matrix = true;
    }

    pub fn translate(&mut self, tx: f32, ty: f32) {
        self.make_complex();
        self.m_trans_x1 += tx;
        self.m_trans_y1 += ty;
    }

    pub fn rotate_rad(&mut self, rot: f32) {
        self.m_rot += rot;
        self.m_have_rot = true;
        self.m_need_calc_matrix = true;
    }

    pub fn rotate_deg(&mut self, rot: f32) {
        self.rotate_rad(rot * std::f32::consts::PI / 180.0);
    }

    pub fn scale(&mut self, sx: f32, sy: f32) {
        self.make_complex();
        self.m_scale_x *= sx;
        self.m_scale_y *= sy;
        self.m_have_scale = true;
    }

    fn make_complex(&mut self) {
        self.m_complex = true;
        self.m_need_calc_matrix = true;
    }

    fn calc_matrix(&mut self) {
        if !self.m_need_calc_matrix {
            return;
        }
        self.m_matrix.matrix.load_identity();
        self.m_matrix.translate(self.m_trans_x1, self.m_trans_y1);
        if self.m_have_rot {
            self.m_matrix.rotate_rad(self.m_rot);
        }
        if self.m_have_scale {
            self.m_matrix.scale(self.m_scale_x, self.m_scale_y);
        }
        self.m_matrix.translate(self.m_trans_x2, self.m_trans_y2);
        self.m_need_calc_matrix = false;
    }

    pub fn get_matrix(&mut self) -> &SexyTransform2D {
        self.calc_matrix();
        &self.m_matrix
    }
}

impl Default for Transform {
    fn default() -> Self {
        Transform::new()
    }
}
