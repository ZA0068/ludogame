use csv;

fn export_vec(vector: Vec<f64>) {
    let mut wtr = csv::Writer::from_path("test.csv").unwrap();
    let row = vector.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(",");
    wtr.write_record(&[row]).unwrap();
    wtr.flush().unwrap();
}
fn main() {
    let vec = vec![1.2, 2.5, 3.7];
    export_vec(vec);
}
