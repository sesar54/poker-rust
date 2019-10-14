

#[cfg(test)]
mod card {

    #[test]
    fn test() {

        let mut vec2: Vec::<Vec<i32>>;

        vec2 = Vec::new();

        vec2.last_mut();
        //vec2.last_mut().unwrap_or(&mut Vec::<i32>::new()).push(4);
        //vec2.last_mut().get_or_insert_with(|x| x.push(Vec::<i32>::new())).push(6);

        //vec2.last_mut().get_or_insert(&mut Vec::<i32>::new()).push(4);

        println!("Vector contains {:?}", vec2);

    }

}