//1108	Defanging an IP Address    

pub fn defang_i_paddr(address: String) -> String {
    return address.replace(".", "[.]") as String;
}