use std::vec::Vec;

#[derive(Debug)]
pub struct LinkList<T> {
    len: usize,
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> LinkList<T> {
    pub fn new() -> Self {
        LinkList { len: 0, head: None }
    }

    pub fn from(list: Vec<T>) -> Self {
        let mut root = Self::new();
        for i in list {
            root.push(i);
        }
        return root;
    }

    pub fn len(&self) -> usize {
        return self.len;
    }

    pub fn index(&self, i: usize) -> Option<&T> {
        if i >= self.len {
            return None;
        }
        let mut pos = 0;
        let mut iter = self.head.as_ref();
        while iter.is_some() {
            if pos == i {
                return Some(&iter.as_ref().unwrap().elem);
            }
            pos += 1;
            iter = iter.unwrap().next.as_ref();
        }

        return None;
    }

    pub fn push(&mut self, elem: T) {
        // 先判断head是否为空，为空则赋值给head
        if self.len == 0 {
            self.head = Some(Box::new(Node {
                elem: elem,
                next: None,
            }));
            self.len += 1;
            return;
        }

        // 有一个节点，则在后面增加1个节点
        if self.len == 1 {
            let head = self.head.as_mut();
            head.unwrap().next = Some(Box::new(Node {
                elem: elem,
                next: None,
            }));
            self.len += 1;
            return;
        }

        // 找到最后一个迭代器，将elem赋值给 iter.next
        let mut iter = self.head.as_mut().unwrap();
        while iter.next.is_some() {
            iter = iter.next.as_mut().unwrap();
        }
        iter.next = Some(Box::new(Node {
            elem: elem,
            next: None,
        }));
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        if self.len == 1 {
            let result = self.head.take().unwrap().elem;
            self.head = None;
            self.len -= 1;
            return Some(result);
        }

        let mut iter = self.head.as_mut().unwrap();

        // iter为倒数第二个时，结束while循环
        while !(iter.next.is_some() && iter.next.as_ref().unwrap().next.is_none()) {
            iter = iter.next.as_mut().unwrap();
        }

        // 移除最后一个节点
        let result = iter.next.take().unwrap().elem;
        iter.next = None;
        self.len -= 1;
        return Some(result);
    }
}

impl<T> Drop for LinkList<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_linklist_new() {
        let linklist = LinkList::<usize>::new();
        assert_eq!(true, linklist.head.is_none());
    }

    #[test]
    fn test_linklist_from() {
        let list = LinkList::<usize>::from(vec![2, 3, 4]);
        assert_eq!(3, list.len());
        assert_eq!(2, list.head.as_ref().unwrap().elem);
        assert_eq!(3, list.head.as_ref().unwrap().next.as_ref().unwrap().elem);
        assert_eq!(
            4,
            list.head
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .elem
        );
        assert_eq!(
            true,
            list.head
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .next
                .is_none()
        );
    }

    #[test]
    fn test_linklist_len() {
        let mut list = LinkList::<usize>::new();
        assert_eq!(0, list.len());

        list = LinkList::<usize>::from(vec![3, 4, 5]);
        assert_eq!(3, list.len());
    }

    #[test]
    fn test_linklist_index() {
        let list = LinkList::<usize>::from(vec![2, 3, 4, 5]);
        assert_eq!(&2, list.index(0).unwrap());
        assert_eq!(&5, list.index(3).unwrap());
        assert_eq!(true, list.index(4).is_none());
    }

    #[test]
    fn test_linklist_push() {
        let mut list = LinkList::<usize>::new();
        list.push(4);
        assert_eq!(1, list.len());
        assert_eq!(4, list.head.as_ref().unwrap().elem);

        list.push(5);
        assert_eq!(2, list.len());
        assert_eq!(5, list.head.as_ref().unwrap().next.as_ref().unwrap().elem);

        list.push(6);
        assert_eq!(3, list.len());
        assert_eq!(
            6,
            list.head
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .elem
        );
    }

    #[test]
    fn test_linklist_pop() {
        let mut list = LinkList::<usize>::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut t = list.pop();
        assert_eq!(3, t.unwrap());
        t = list.pop();
        assert_eq!(2, t.unwrap());
        t = list.pop();
        assert_eq!(1, t.unwrap());
        t = list.pop();
        assert_eq!(true, t.is_none());
    }
}
