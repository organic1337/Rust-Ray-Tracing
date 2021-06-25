use crate::engine::hittables::hittable::{Hittable, HitRecord};
use crate::engine::Ray;

pub struct HittableCollection<'a> {
    hittable_list: Vec<&'a dyn Hittable<'a>>
}


impl<'a> HittableCollection<'a> {
    pub fn new() -> HittableCollection<'a> {
        HittableCollection {
            hittable_list: Vec::new()
        }
    }

    pub fn add(&mut self, hittable: &'a dyn Hittable<'a>) {
        self.hittable_list.push(hittable);
    }

    pub fn clear(&mut self) {
        self.hittable_list.clear();
    }
}

impl<'a> Hittable<'a> for HittableCollection<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
        let mut smallest_distance = t_max;
        let mut result = None;

        for hittable in &self.hittable_list {
            let hit_result = hittable.hit(ray, t_min, smallest_distance);
            match hit_result {
                Some(hit_record) => {
                    smallest_distance = hit_record.t;
                    result = Some(hit_record);
                },
                _ => {}
            };
        }

        result
    }
}