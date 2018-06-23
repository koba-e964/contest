#include <iostream>
#include <string>
#include <algorithm>
#include <utility>
#include <vector>

using namespace std;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

struct node {
  node *ch[10];
  ll val;
  ll ma;
  ll lazy;
  node() {
    val = 0;
    ma = 0;
    lazy = 0;
    REP(i, 0, 10) ch[i] = NULL;
  }
  void add(const string &a, int pos, ll v) {
    if (pos >= (int) a.size()) {
      lazy += v;
      ma = ::max(val, ma);
      return;
    }
    // propagate
    int idx = a[pos] - '0';
    if (lazy != 0) {
      REP(idx, 0, 10) {
	if (ch[idx] == NULL) ch[idx] = new node;
      }
      REP(i, 0, 10) {
	if (ch[i]) {
	  ch[i]->lazy += lazy;
	}
      }
    }
    ma += lazy;
    val += lazy;
    lazy = 0;
    if (ch[idx] == NULL) ch[idx] = new node;
    ch[idx]->add(a, pos + 1, v);
    ma = ::max(ma, ch[idx]->max());
  }
  ll max(void) const {
    return lazy + ma;
  }
  void debug(string s = "") const {
    cerr << "[" << s << "]:" << val << "(m:" << ma << ") (l:" << lazy << ")"
	 << endl;
    REP(i, 0, 10) {
      if (ch[i]) {
	ch[i]->debug(s + (char)('0' + i));
      }
    }
  }
} root;

int main(void) {
  int n;
  cin >> n;
  REP(i, 0, n) {
    string a;
    ll b;
    cin >> a >> b;
    reverse(a.begin(), a.end());
    root.add(a, 0, b);
    cout << root.max() << endl;
    // root.debug();
  }
}
