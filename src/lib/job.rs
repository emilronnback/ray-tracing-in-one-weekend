use std::vec::Vec;

#[derive(PartialEq)]
pub struct Job {
    pub height_range: std::ops::Range<usize>,
    pub width_range: std::ops::Range<usize>,
}
impl Job {
    pub fn new(height_range: std::ops::Range<usize>, width_range: std::ops::Range<usize>) -> Job {
        Job {
            height_range,
            width_range,
        }
    }
}

pub fn create_jobs(height: usize, width: usize) -> Vec<Job> {
    let mut jobs = Vec::new();
    let height = (0..height).collect::<Vec<_>>();
    let height_chunks = height.chunks(1).map(|s| {
        let start = *s.first().unwrap();
        let end = *s.last().unwrap() + 1;
        start..end
    });
    for h in height_chunks.rev() {
        jobs.push(Job::new(h.clone(), 0..width));
    }
    jobs
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn construct_jobs() {
        let first = Job::new(0..1, 0..50);
        let last = Job::new(99..100, 0..50);

        let jobs = create_jobs(100, 50);
        for j in &jobs {
            println!(
                "{} {} : {} {}",
                j.height_range.start, j.height_range.end, j.width_range.start, j.width_range.end
            );
        }
        assert_eq!(50, jobs.len());
        assert!(jobs.contains(&first));
        assert!(jobs.contains(&last));
    }

    #[test]
    fn construct_jobs10() {
        let first = Job::new(0..1, 0..100);
        let last = Job::new(199..200, 0..100);

        let jobs = create_jobs(200, 100);
        for j in &jobs {
            println!(
                "{} {} : {} {}",
                j.height_range.start, j.height_range.end, j.width_range.start, j.width_range.end
            );
        }
        assert_eq!(20, jobs.len());
        assert!(jobs.contains(&first));
        assert!(jobs.contains(&last));
    }
}
