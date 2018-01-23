/*
 * Permanent array.
 * Warning: NO GARBAGE COLLECTION!!!
 * Parameter: B, D (2^(B * D) > (maximum index) should hold)
 * Header Requirement: cassert
 * Verified by: CF358-2D (http://codeforces.com/contest/707/submission/34492976)
 */
template<class T>
class IntMap {
public:
  IntMap(): root(nullptr) {}
  T get(unsigned int key) const {
    return get_sub(key, this->root, D - 1);
  }
  void set(unsigned int key, T value) {
    set_sub(key, this->root, value, D - 1);
  }
private:
  static const int B=3; // parameter
  static const int D=7; // parameter
  static const int BLOCK_SIZE=1<<B;
  struct node {
    node* child[BLOCK_SIZE];
    T elem;
    node() : elem() {
      for (int i = 0; i < BLOCK_SIZE; ++i) { this->child[i] = nullptr; }
    }
    node(const node &o): elem(o.elem) {
      for (int i = 0; i < BLOCK_SIZE; ++i) { this->child[i] = o.child[i]; }
    }
  };
  node *root;
  const T get_sub(unsigned int key, node *v, int depth) const {
    if (v == nullptr) { return T(); }
    if (depth == -1) { return v->elem; }
    assert (key >> (depth * B) < BLOCK_SIZE);
    unsigned int idx = key >> (depth * B);
    return get_sub(key & ((1U << (depth * B)) - 1), v->child[idx], depth - 1);
  }
  void set_sub(unsigned int key, node *&v, T value, int depth) {
    if (v == nullptr) {
      v = new node;
    } else {
      v = new node(*v);
    }
    if (depth == -1) {
      v->elem = value;
      return;
    }
    assert (key >> (depth * B) < BLOCK_SIZE);
    unsigned int idx = key >> (depth * B);
    // create a shallow copy of v->child[idx]
    set_sub(key & ((1U << (depth * B)) - 1), v->child[idx], value, depth - 1);
  }
};
