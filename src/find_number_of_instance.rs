fn find(s:&str,c:char)->Option<usize>{
    let mut num:usize=0;
//    let s=String::from(s);
    for i in s.chars(){
        if i==c{
            num+=1;
        }
    }
    Some(num)
}

fn find_index(s:&str,c:char)->Option<usize>{
    for (offset,ch) in s.char_indices(){
        if ch==c{
            return Some(offset);
        }
    }
    None
}
fn find_file_entension(file_name:&str)->Option<&str>{
    match find_index(file_name,'.'){
        Some(i)=>return Some(&file_name[i+1..]),
        None=>None,
    }
}
#[test]
fn test_find_file_extension(){
    assert_eq!(find_file_entension("foo.rs").unwrap(),"rs" );
}


#[test]
fn test_find(){
    assert_eq!(3,find("aaadf",'a').unwrap());
    assert_eq!(8,find("aaadfdfdsafdasaddfdsaafd",'d').unwrap());
}