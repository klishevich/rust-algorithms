pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

// enum Link {
//     Empty,
//     More(Box<Node>)
// }

struct Node<T> {
    elem: T,
    next: Link<T>
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: Link::None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new (Node {
            elem: elem,
            next: self.head.take()
        });

        self.head = Link::Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Link::Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

// ------------- IntoIter - T -------------------
pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}

// ---------------- Iter - &T --------------------
// lifetime
// Iter is generic over *some* lifetime, it doesn't care
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

// No lifetime here, List doesn't have any associated lifetimes
impl<T> List<T> {
    // We declare a fresh lifetime here for the *exact* borrow that
    // creates the iter. Now &self needs to be valid as long as the
    // Iter is around.
    // pub fn iter<'a>(&'a self) -> Iter<'a, T> {
    //     Iter { next: self.head.as_deref() }
    // }
    // pub fn iter(& self) -> Iter<T> {
    //     Iter { next: self.head.as_deref() }
    // }
    pub fn iter(& self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }
}

// We *do* have a lifetime here, because Iter has one that we need to define
impl<'a, T> Iterator for Iter<'a, T> {
    // Need it here too, this is a type declaration
    type Item = &'a T;

    // None of this needs to change, handled by the above.
    // Self continues to be incredibly hype and amazing
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            // Just for completeness' sake, we could give it a different hint with the turbofish:
            // self.next = node.next.as_ref().map::<&Node<T>, _>(|node| &node);
            &node.elem
        })
    }
}

// --------------- IterMut - &mut T ---------------------
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate List
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push more elements
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // List exhaustion 
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list: List<i32> = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        list.push(1); list.push(2); list.push(3);
        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|value| {
            *value = 42
        });
    
        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}