# Object-Oriented Programming in Rust

Many languages support object-oriented programming, they have classes, interfaces, and inheritances. While the classes stands for a collection of data and the interfaces determines a set of methods. 

In Rust, struct is a collection of data and the trait is a set of methods which determines what should a struct do, and a trait can be implemented by multiple structs. We can combine the data set and the behavior of the data into one concept which is called an *object*.

Rust doesn't have the concept of "inheritance", try thinking what we need the inheritance for. A base class extended by multiple classes must have many behaviors that its derived classes need in common. For instance, for the `Animal` class, it's an abstract class, we can derive many real animals from the `Animal` class like `Dog`, `Cat` or `Chicken`. These animals all eat, if we want to have them all eat, instead of calling every instance one by one, we can define a collection of `Animal` type which contains all these animal instances. So we can iterate the collection and just the `eat` method. Though the `eat` method may implemented differently in different classes, they all eat.

Let's see how Rust implement this idea without inheritance. As there is no inheritance, all these real animals don't have a common base class, how can we collect them in a data structure that normally requires all elements are the same type? 

We all know a trait defines a set of methods, but we can also see a trait as an object, or as a dynamically sized type. But we still cannot just use the trait as the type annotation in a data structure cause it's dynamically sized type. We have to wrap it as a smart pointer, like `Box<dyn Eat>`. 

Back to the example, in Rust, if we want to have all animals eat, we can do it in this way:

```rust
pub struct Farm {
    pub animals: Vec<Box<dyn Eat>>,
}

impl Farm {
    pub fn eat(&self) {
        for animal in self.animals.iter() {
            animal.eat();
        }
    }
}
```

You may wonder why don't we use the trait bounds. The trait bounds are suitable for functions, but for containers, we can't put put all kinds of types that all implement a specific type into one container. That's where the limitation is. 

#### Object Safety Is Required for Trait Objects

You can only make *object-safe* traits into trait objects. A trait is object safe if all the methods defined in the trait have the following properties:

-   The return type isn't `Self`
-   There are no generic type parameters

The `Seld` keyword is an alias for the type we're implementing the traits or methods on. Trait objects must be object safe because once you've used a trait object. Rust no longer knows the concrete type that's implementing the trait. If a trait method returns the concrete `Self` type, but a trait object forgets the exact type that `Self` is, there is no way the method can use the original concrete type. The same is true of generic type become part of the type that implements the trait. When the type is forgotten through the use of a trait object, there is no way to know what types to fill in the generic type parameters with. 

