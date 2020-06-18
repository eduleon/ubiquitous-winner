use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::LinkedList;

use lazy_static::lazy_static;

extern crate ureq;
use ureq::{SerdeMap, SerdeValue};

extern crate chrono;
use chrono::NaiveDateTime;
/*
pub enum Queries {
    SMA50,
    SMA100,
    SMA200,
}

pub enum Types {
    USD,
    EURO,
    COBRE,
    PLATA,
    ORO,
}
*/

// TODO unsorted :/
lazy_static! {
    static ref INDICATORS: HashMap<&'static str, BTreeMap<String, f64>> = {
        let indicator_names = vec!["cobre", "plata", "oro"];
        load_indicators(indicator_names)
    };
}

fn load_indicators(
    indicator_names: Vec<&'static str>,
) -> HashMap<&'static str, BTreeMap<String, f64>> {
    println!("[STATIC] Loading {} data", json!(indicator_names));
    let mut indicators_map = HashMap::new();

    for indi in indicator_names {
        let response_json = get_indicator_as_json(indi);
        let response_map = response_json["values"].as_object().unwrap();
        /*
        let sma_map = get_sma_from_timeseries(response_map, 50);
         */
        let sorted_map = get_sorted_timeseries(response_map);
        indicators_map.insert(indi, sorted_map);
    }

    indicators_map
}

pub fn init()
// -> HashMap<String, HashMap<String, f64>>
{
    println!("Init indicators data");
    println!(
        "Init indicators! Example 'oro': {}",
        json!(INDICATORS.get("oro"))
    );
    //loop {
    //println!("Periodic call");
    //thread::sleep(Duration::from_secs(3));

    //sender.send(sma_map).unwrap();
    //}
    //indicator_series
}

// TODO 50SMA, 100SMA...
pub fn get_sma(element: &str) -> BTreeMap<String, f64> {
    INDICATORS.get(element).unwrap().clone()
}

// TODO async calls
fn get_indicator_as_json(element: &str) -> serde_json::Value {
    let api_uri = format!("https://www.indecon.online/values/{}", element);
    println!("Calling to: {}", api_uri);
    let response = ureq::get(&api_uri).call();
    response.into_json().unwrap()
}

fn get_sorted_timeseries(values_map: &SerdeMap<String, SerdeValue>) -> BTreeMap<String, f64> {
    let mut sorted_map = BTreeMap::new();
    for (key, value) in values_map {
        sorted_map.insert(String::from(key), value.as_f64().unwrap());
    }
    println!("btreemap {}", json!(sorted_map));
    sorted_map
}

fn get_sma_from_timeseries(
    values_map: &SerdeMap<String, SerdeValue>,
    period: usize,
) -> HashMap<String, f64> {
    let mut window: LinkedList<f64> = LinkedList::new();
    let mut prefix_sum = 0.0;

    let mut new_map = HashMap::new();
    for (key, value) in values_map {
        let float_value = value.as_f64().unwrap();

        window.push_back(float_value);
        prefix_sum += float_value;
        if window.len() > period {
            prefix_sum -= window.pop_front().unwrap();
        }

        let sma = prefix_sum / window.len() as f64;
        //println!("key: {}, value: {}, counter {}, prefix_sum: {}, sma: {}", key, value, counter, prefix_sum, sma);
        let date_time = NaiveDateTime::from_timestamp(key.parse::<i64>().unwrap(), 0);
        let date = date_time.format("%Y-%m-%d").to_string();
        new_map.insert(date, sma);
    }
    new_map
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
        let mut expected_sma_values = HashMap::with_capacity(5);
        expected_sma_values.insert("2019-01-02", 2.71);
        expected_sma_values.insert("2019-01-03", 2.7);
        expected_sma_values.insert("2019-01-04", 2.64);
        expected_sma_values.insert("2019-01-07", 2.59);
        expected_sma_values.insert("2019-01-08", 2.68);
    */

    #[test]
    fn invalid_sma_value() {
        let mut some_element_values = HashMap::with_capacity(5);
        some_element_values.insert("1546387200", 2.71);
        some_element_values.insert("1546473600", 2.7);
        some_element_values.insert("1546560000", 2.64);
        some_element_values.insert("1546819200", 2.59);
        some_element_values.insert("1546905600", 2.68);
        let sma_period = 50;
        let unexpected_sma_value = 99999.9999999999999;

        let sma_values =
            get_sma_from_timeseries(json!(some_element_values).as_object().unwrap(), sma_period);
        let specific_sma_value = sma_values.get("2019-01-02").unwrap();

        assert_ne!(
            &unexpected_sma_value, specific_sma_value,
            "SMA values were equal"
        );
    }

    #[test]
    fn valid_sma_value() {
        let mut some_element_values = HashMap::with_capacity(5);
        some_element_values.insert("1546387200", 2.71);
        some_element_values.insert("1546473600", 2.7);
        some_element_values.insert("1546560000", 2.64);
        some_element_values.insert("1546819200", 2.59);
        some_element_values.insert("1546905600", 2.68);
        let sma_period = 50;
        let expected_sma_value = 2.664;

        let sma_values =
            get_sma_from_timeseries(json!(some_element_values).as_object().unwrap(), sma_period);
        let specific_sma_value = sma_values.get("2019-01-08").unwrap();

        assert_eq!(
            &expected_sma_value, specific_sma_value,
            "SMA values were different"
        );
    }

    #[test]
    fn valid_sma_2_value() {
        let mut some_element_values = HashMap::with_capacity(5);
        some_element_values.insert("1546387200", 2.71);
        some_element_values.insert("1546473600", 2.7);
        some_element_values.insert("1546560000", 2.64);
        some_element_values.insert("1546819200", 2.59);
        some_element_values.insert("1546905600", 2.68);
        let sma_period = 2;
        let expected_sma_value = 2.635;

        let sma_values =
            get_sma_from_timeseries(json!(some_element_values).as_object().unwrap(), sma_period);
        let specific_sma_value = sma_values.get("2019-01-08").unwrap();

        assert_eq!(
            &expected_sma_value, specific_sma_value,
            "SMA values were different"
        );
    }

    #[test]
    fn valid_sma_3_value() {
        let mut some_element_values = HashMap::with_capacity(5);
        some_element_values.insert("1546387200", 2.71);
        some_element_values.insert("1546473600", 2.7);
        some_element_values.insert("1546560000", 2.64);
        some_element_values.insert("1546819200", 2.59);
        some_element_values.insert("1546905600", 2.68);
        let sma_period = 3;
        let expected_sma_value = 2.636666666666667;

        let sma_values =
            get_sma_from_timeseries(json!(some_element_values).as_object().unwrap(), sma_period);
        let specific_sma_value = sma_values.get("2019-01-08").unwrap();

        assert_eq!(
            &expected_sma_value, specific_sma_value,
            "SMA values were different"
        );
    }
}
