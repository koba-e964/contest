#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

struct node {
  int mul; // meaningful only if mask = 1
  node *child[2];
} top;

void add_node(node *n, int x, int label, int mask) {
  assert (x >= label && x < label + mask);
  if (mask == 1) {
    n->mul++;
    return;
  }
  int m = mask / 2;
  int idx = (x - label) / m;
  assert (idx == 0 || idx == 1);
  if (n->child[idx] == NULL) {
    node *p = new node;
    p->mul = 0;
    p->child[0] = NULL;
    p->child[1] = NULL;
    //cerr << "creating: " << p->label << " " << p->mask << endl;
    n->child[idx] = p;
  }
  add_node(n->child[idx], x, label + idx * m, m);
}


/* true -> the node was deleted */
bool remove_node(node* n, int x, int label, int mask) {
  if (mask == 1) {
    n->mul--;
    return n->mul == 0;
  }
  int m = mask / 2;
  int idx = (x - label) / m;
  assert (idx == 0 || idx == 1);
  assert (n->child[idx]);
  bool removed = remove_node(n->child[idx], x, label + idx * m, m);
  if (removed) {
    n->child[idx] = NULL;
  }
  return n->child[0] == NULL && n->child[1] == NULL;
}

int query_node(node *n, int x, int label, int mask) {
  //cerr << "query_node: " << n->label << " " << n->mask << " " << x << endl;
  if (mask == 1) {
    return label;
  }
  assert (n->child[0] || n->child[1]);
  if (n->child[0] != 0 ^ n->child[1] != 0) {
    if (n->child[0]) {
      return query_node(n->child[0], x, label, mask / 2);
    } else {
      return query_node(n->child[1], x, label + mask / 2, mask / 2);
    }
  }
  int m = mask / 2;
  int idx = (x & m) ? 0 : 1;
  return query_node(n->child[idx], x, label + idx * m, m);
}


int main(void){
  int q;
  cin >> q;
  int whole = 1 << 30;

  top.child[0] = NULL;
  top.child[1] = NULL;
  add_node(&top, 0, 0, whole);
  
  REP(loop_cnt, 0, q) {
    string s;
    int x;
    cin >> s >> x;
    if (s[0] == '+') {
      add_node(&top, x, 0, whole);
    }
    if (s[0] == '-') {
      remove_node(&top, x, 0, whole);
    }
    if (s[0] == '?') {
      cout << (query_node(&top, x, 0, whole) ^ x) << endl;
    }
  }
}
