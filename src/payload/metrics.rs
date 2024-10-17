
use rand::Rng; // For generating random numbers
use serde::{Serialize, Deserialize};
use struct_iterable::Iterable;

#[derive(Debug, Clone,Deserialize,Serialize)]
#[serde(untagged)]
pub enum MetricValue {
    Int(usize),   // or you can use i32, usize, etc.
    Float(f64),
}
#[derive(Debug, Serialize, Deserialize,Iterable)]
pub struct Metrics {
    fp2: usize,
    browser: usize,
    capabilities: usize,
    gpu: usize,
    dnt: usize,
    math: usize,
    screen: usize,
    navigator: usize,
    auto: usize,
    stealth: usize,
    subtle: usize,
    canvas: usize,
    formdetector: usize,
    be: usize,
}

impl Metrics {
    pub fn new() -> Metrics {
        let mut rng = rand::thread_rng();
        Metrics {
            fp2: 1,
            browser: 0,
            capabilities: 0,
            gpu: rng.gen_range(13..=16), 
            dnt: 0,
            math: 0,
            screen: 0,
            navigator: 0,
            auto: 0,
            stealth: 1,
            subtle: 0,
            canvas: rng.gen_range(13..=16), // Generates a value between 13 and 16
            formdetector: 0,
            be: 0,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MetricData
{

    name : String,
    value : MetricValue,
    unit : String
}

impl MetricData  {
    pub fn metric_to_metric_data(collected_metrics : Metrics) -> Vec<MetricData> {
        let mut rng = rand::thread_rng();
        let mut metrics : Vec<MetricData> = vec![];
        metrics.push(MetricData { name: 2.to_string(), value: MetricValue::Float(rng.gen_range(0.25..0.41)), unit: 2.to_string() });
        let mut counter = 0;
        for (_, value) in collected_metrics.iter() {
            // println!("{:?}",value.downcast_ref());
            metrics.push(MetricData { 
            name: format!("{}",100+counter), 
            value: MetricValue::Int(*value.downcast_ref::<usize>().expect("Error")) , 
            unit: 2.to_string() 
            });
            counter +=1;
        }

        metrics.push(MetricData { name: 3.to_string(), value: MetricValue::Float(rng.gen_range(1.0..10.0)), unit: 2.to_string() });
        metrics.push(MetricData { name: 7.to_string(), value: MetricValue::Int(1), unit: 4.to_string() });
        metrics.push(MetricData { name:1.to_string(), value: MetricValue::Float(rng.gen_range(16.0..23.0)), unit: 2.to_string() });
        metrics.push(MetricData { name:4.to_string(), value: MetricValue::Float(rng.gen_range(7.0..8.0)), unit: 2.to_string() });
        metrics.push(MetricData { name:5.to_string(), value: MetricValue::Float(rng.gen_range(0.1..0.2)), unit: 2.to_string() });
        metrics.push(MetricData { name:6.to_string(), value: MetricValue::Float(rng.gen_range(25.0..100.0)), unit: 2.to_string() });
        metrics.push(MetricData { name:0.to_string(), value: MetricValue::Float(rng.gen_range(75.1..90.2)), unit: 2.to_string() });
        metrics.push(MetricData { name: 8.to_string(), value: MetricValue::Int(1), unit: 4.to_string() });
        metrics
        
    } 
}

#[cfg(test)]
mod tests {
    use super::{MetricData,Metrics};
    use std::time::Instant;
    #[test]
    fn test_metric_data(){
        let start = Instant::now();
        let metrics = Metrics::new();
        let metric_data = MetricData::metric_to_metric_data(metrics);
        let serialized = serde_json::to_string(&metric_data).unwrap();
        println!("{serialized}");
        println!("Time : {:?}",start.elapsed());

    }
}