#pragma once

// Multiple inheritance. Rust treats each derived class independently.
// If you need cross-class polymorphism across the inheritance chain,
// hicc's #[interface] + @make_proxy is an option, but for a small demo
// we just expose the derived class's combined method set.

class Drawable {
public:
    virtual ~Drawable() = default;
    virtual void draw() const = 0;
};

class Serializable {
public:
    virtual ~Serializable() = default;
    virtual int byte_size() const = 0;
};

class Sprite : public Drawable, public Serializable {
public:
    explicit Sprite(int w, int h) : w_(w), h_(h) {}
    void draw() const override {}
    int  byte_size() const override { return w_ * h_ * 4; }
    int  width() const { return w_; }
    int  height() const { return h_; }
private:
    int w_, h_;
};

Sprite* sprite_new(int w, int h);
void    sprite_free(Sprite* s);
