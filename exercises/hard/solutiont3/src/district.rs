use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use serde_json::Value;

pub fn count_provinces() -> String {
    // 读取并解析JSON文件
    let mut file = File::open("district.json").expect("Failed to open district.json file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let json: Value = serde_json::from_str(&contents).unwrap();

    let mut results = Vec::with_capacity(5);

    // 处理每个批次
    for i in 1..=5 {
        let batch = &json[i.to_string()];
        if batch.is_object() {
            let count = count_connected_components(batch);
            results.push(count);
        }
    }

    // 将结果转换为指定格式的字符串
    results.iter()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn count_connected_components(batch: &Value) -> i32 {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    
    // 收集所有城市间的连接
    if let Some(obj) = batch.as_object() {
        for (city, connections) in obj {
            if let Some(conn_array) = connections.as_array() {
                for conn in conn_array.iter().filter_map(|v| v.as_str()) {
                    if city != conn {
                        graph.entry(city.clone())
                            .or_insert_with(HashSet::new)
                            .insert(conn.to_string());
                        graph.entry(conn.to_string())
                            .or_insert_with(HashSet::new)
                            .insert(city.clone());
                    }
                }
            }
        }
    }
    
    // 深度优先搜索 (DFS) 计算连通分量数量
    let mut visited = HashSet::new();
    let mut provinces = 0;

    for city in graph.keys() {
        if !visited.contains(city) {
            dfs(&graph, city, &mut visited);
            provinces += 1;
        }
    }

    provinces
}

fn dfs(graph: &HashMap<String, HashSet<String>>, city: &str, visited: &mut HashSet<String>) {
    if visited.contains(city) {
        return;
    }
    
    visited.insert(city.to_string());
    if let Some(neighbors) = graph.get(city) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                dfs(graph, neighbor, visited);
            }
        }
    }
}
