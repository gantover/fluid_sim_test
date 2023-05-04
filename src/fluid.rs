pub const DENSITY: f64 = 1.;
pub const DELTA: f64 = 1./30.;
pub const H: f64 = 0.1;

pub struct Matrice {
    pub rows: Vec<Vec<f64>>
}

impl Matrice {
    fn new(rows: usize, cols: usize, default: f64) -> Matrice {
        let mut default_rows = Vec::new();
        let mut default_col: Vec<f64> = Vec::new();
        for _ in 0..cols {
            default_col.push(default)
        };
        for _ in 0..rows {
            default_rows.push(default_col.clone());
        };
        Matrice { rows: default_rows }
    }
    fn show(&self) {
        for row in self.rows.clone() {
            for col in row {
                print!("{} ", col);
            };
            print!("\n");
        };
    } 
}

pub struct Fluid {
    pub size_x: usize,
    pub size_y: usize,
    pub u: Matrice,
    pub v: Matrice,
    pub s: Matrice,
    pub p: Matrice,
}

impl Fluid {
    pub fn new(size_x: usize, size_y: usize) -> Fluid {
        let u: Matrice = Matrice::new(size_x, size_y, 0.);
        let v: Matrice = Matrice::new(size_x, size_y, 0.);
        let mut s: Matrice = Matrice::new(size_x, size_y, 1.);
        let p: Matrice = Matrice::new(size_x, size_y, 0.);
        for col_id in 0..size_x {
            s.rows[0][col_id] = 0.;
            s.rows[size_y-1][col_id] = 0.;
        }
        for row_id in 0..size_y {
            s.rows[row_id][0] = 0.;
            s.rows[row_id][size_x-1] = 0.;
        }
        for row_id in size_y/3..size_y-size_y/3 {
            s.rows[row_id][8] = 0.;
        }
        Fluid { size_x, size_y, u, v, s, p }
    }

    pub fn apply_gravity(&mut self) {
        for row_id in 0..self.size_y {
            for col_id in 0..self.size_x {
                if self.v.rows[row_id][col_id] != 0. {
                    self.v.rows[row_id][col_id] = self.v.rows[row_id][col_id] - 9.81 * DELTA ;
                }
            }
        }
    }

    pub fn excitation(&mut self) {
        for row_id in self.size_y/4..self.size_y-self.size_y/4 {
            self.v.rows[row_id][1] = -1.;
        }
    }
    
    pub fn calculate(&mut self) {
        for i in 1..self.size_y-1 {
            for j in 1..self.size_x-1 {
                let div = self.u.rows[i+1][j] - self.u.rows[i][j] + self.v.rows[i][j+1] - self.v.rows[i][j];
                let s = self.s.rows[i+1][j] + self.s.rows[i-1][j] + self.s.rows[i][j+1] + self.s.rows[i][j-1];
                self.u.rows[i][j] = self.u.rows[i][j] + div * self.s.rows[i-1][j] / s;
                self.u.rows[i+1][j] = self.u.rows[i+1][j] - div * self.s.rows[i+1][j] / s;
                self.v.rows[i][j] = self.v.rows[i][j] + div * self.s.rows[i][j-1] / s;
                self.v.rows[i][j+1] = self.v.rows[i][j+1] - div * self.s.rows[i][j+1] / s;
                self.p.rows[i][j] = self.p.rows[i][j] + (div * DENSITY * H) / (self.s.rows[i][j] * DELTA);
            }
        }
    }
}
