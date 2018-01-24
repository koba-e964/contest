/*
 * Permanent array.
 * Warning: NO GARBAGE COLLECTION!!!
 * Parameter: B, D (2^(B * D) > (maximum index) should hold)
 * Header Requirement: cassert
 * Verified by: CF368-2D (http://codeforces.com/contest/707/submission/34504401)
 */
template<class T, // int -> T
	 int B = 3, // parameter
	 int D = 7 // parameter
	 >
class IntMap {
public:
  IntMap(): root(nullptr) {}
  T get(unsigned int key) const {
    return get_sub(key, this->root, D - 1);
  }
  IntMap<T, B, D> set(unsigned int key, T value) const {
    IntMap<T, B, D> ret(*this);
    ret.set_sub(key, ret.root, value, D - 1);
    return ret;
  }
private:
  static_assert (B * D <= 8 * sizeof(int), "B * D > 8 * sizeof(int)");
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
