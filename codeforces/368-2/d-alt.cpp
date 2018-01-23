#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m, q;
  cin >> n >> m >> q;
  int tot[100001] = {};
  IntMap<int> t[100001];
  IntMap<int> shelf[100001];

  REP(i, 1, n + 1) {
    t[0].set(i, 0); // is shelf i inverted?
    t[0].set(i+n, 0); // #books in shelf i
    REP(j, 1, m + 1) {
      shelf[0].set(i * m + j, 0);
    }
  }
  REP(time, 1, q + 1) {
    int kind;
    cin >> kind;
    if (kind == 4) {
      int k;
      cin >> k;
      t[time] = t[k];
      shelf[time] = shelf[k];
      tot[time] = tot[k];
    } else {
      t[time] = t[time - 1];
      shelf[time] = shelf[time - 1];
      tot[time] = tot[time - 1];
      if (kind == 1 || kind == 2) {
	int i, j;
	cin >> i >> j;
	int goal = 2 - kind;
	int flip = t[time].get(i);
	int cur = shelf[time].get(i * m + j);
	if ((flip ^ cur) != goal) {
	  int nc = flip ^ goal;
	  shelf[time].set(i * m + j, nc);
	  t[time].set(i + n, t[time].get(i + n) + 2 * goal - 1);
	  tot[time] += 2 * goal - 1;
	}
      } else {
	assert (kind == 3);
	int i;
	cin >> i;
	t[time].set(i, 1 - t[time].get(i));
	int old = t[time].get(i + n);
	t[time].set(i + n, m - t[time].get(i + n));
	tot[time] += m - 2 * old;
      }    
    }
    cout << tot[time] << "\n";
    if (0) {
      cerr << "shelf:";
      REP(i, 1, n + 1) {
	REP(j, 1, m + 1) {
	  cerr << " " << shelf[time].get(i * m + j);
	}
      }
      cerr << endl;
    }
  }
}
