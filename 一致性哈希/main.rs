use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::Hasher;

struct CHash {
    vm_node_num: u64,                      //虚拟节点数量
    vm_node_keys: Vec<u64>,                //生成的虚拟节点列表
    nodes: HashMap<u64, String>,           //存放虚拟节点对应的真实节点信息
    my_hash_fn: Option<fn(String) -> u64>, //自定义Hash函数
}

impl CHash {
    /**
     * vm_node_num:要生成的虚拟节点数量
     * my_hash_fn:用户自定义Hash函数
     * */
    fn new(vm_node_num: u64, my_hash_fn: Option<fn(String) -> u64>) -> CHash {
        CHash {
            vm_node_num: vm_node_num,
            vm_node_keys: Vec::new(),
            nodes: HashMap::new(),
            my_hash_fn: my_hash_fn,
        }
    }
    // 添加节点并生成虚拟节点
    fn add_node(&mut self, nodes: Vec<String>) {
        for node in nodes.iter() {
            for i in 0..self.vm_node_num {
                let vm_node = format!("{}{}", i, node);
                let node_hash = match self.my_hash_fn {
                    Some(my_hash) => my_hash(vm_node),
                    _ => CHash::hash_fn(vm_node),
                };
                self.vm_node_keys.push(node_hash);
                self.nodes.insert(node_hash, format!("{}", node));
            }
        }
        self.vm_node_keys.sort();
    }
    // 默认Hash函数
    fn hash_fn(val: String) -> u64 {
        let mut hasher1 = DefaultHasher::new();
        hasher1.write(val.as_bytes());
        hasher1.finish()
    }
    // 根据val选择虚拟节点并寻找对应的真实节点
    fn select_node(&self, val: String) -> String {
        let v_hash = match self.my_hash_fn {
            Some(my_hash) => my_hash(val),
            _ => CHash::hash_fn(val),
        };
        let vm_index = self.vm_node_keys.iter().find(|&&x| -> bool { x >= v_hash });
        let mut node_index: u64 = self.vm_node_keys[0];
        if let Some(&i) = vm_index {
            node_index = i;
        };
        let node = self.nodes.get(&node_index).unwrap();
        node.to_string()
    }
}

fn main() {
    let mut hash_table = CHash::new(3, Some(my_hash_fn));
    hash_table.add_node(vec![
        String::from("2"),
        String::from("4"),
        String::from("6"),
    ]);
    assert_eq!(hash_table.select_node(String::from("1")), "2");
    assert_eq!(hash_table.select_node(String::from("2")), "2");
    assert_eq!(hash_table.select_node(String::from("3")), "4");
    assert_eq!(hash_table.select_node(String::from("11")), "2");
    assert_eq!(hash_table.select_node(String::from("23")), "4");
    assert_eq!(hash_table.select_node(String::from("25")), "6");
    assert_eq!(hash_table.select_node(String::from("35")), "2");
    let mut hash_table_2 = CHash::new(3, None);
    hash_table_2.add_node(vec![
        String::from("127.0.0.1:8081"),
        String::from("127.0.0.1:8082"),
        String::from("127.0.0.1:8083"),
    ]);
    for i in 0..10 {
        println!(
            "{}",
            hash_table_2.select_node(format!("{}-{}", "MyData", i))
        );
    }
}

fn my_hash_fn(val: String) -> u64 {
    let t: u64 = val.parse().expect("无法进行类型转换！");
    t
}
