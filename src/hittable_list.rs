use std::rc::Rc;

use crate::{
    hit::{self, Hittable},
    ray::Ray,
};

#[derive()]
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new_dfl() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn new(object: Rc<dyn Hittable>) -> Self {
        let mut tmp = HittableList::new_dfl();
        tmp.add(object);
        tmp
    }
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut hit::HitRecord) -> bool {
        // let mut temp_rec: HitRecord = HitRecord::new_dfl();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut rec = rec;

        for obj in self.objects.iter() {
            if obj.hit(r, t_min, closest_so_far, &mut rec) {
                hit_anything = true;
                closest_so_far = *rec.t();
            }
        }
        return hit_anything;
    }
}
