#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
const ll inf = 1e15;

struct cons1 {
  ll x, y;
  bool empty;
  cons1(ll x, ll y) : x(x), y(y), empty(false) {}
  cons1(): x(0), y(0), empty(true) {}
  cons1 intersection(cons1 o) const {
    if (empty || o.empty) {
      return cons1();
    }
    ll nx = max(x, o.x);
    ll ny = min(y, o.y);
    if (nx > ny) {
      return cons1();
    }
    return cons1(nx, ny);
  }
};

struct conslist {
  vector<cons1> ls;
  conslist(vector<cons1> ls) {
    REP(i, 0, ls.size()) {
      if (not ls[i].empty) {
	this->ls.push_back(ls[i]);
      }
    }
  }
  conslist intersection(cons1 c) const {
    vector<cons1> ret;
    REP(i, 0, ls.size()) {
      cons1 tmp = ls[i].intersection(c);
      if (not tmp.empty) {
	ret.push_back(tmp);
      }
    }
    return ret;
  }
  conslist intersection(conslist c) const {
    conslist ret(vector<cons1>(0));
    REP(i, 0, c.ls.size()) {
      ret = ret.cup(intersection(c.ls[i]));
    }
    return ret;
  }
  /*
   * this and c need to be disjoint
   */
  conslist cup(conslist c) const {
    vector<cons1> ret(ls.begin(), ls.end());
    REP(i, 0, c.ls.size()) {
      ret.push_back(c.ls[i]);
    }
    return ret;
  }
};

ostream &operator<<(ostream &os, cons1 c) {
  if (c.empty) {
    os << "{}";
  } else {
    os << "[" << c.x << "," << c.y << "]";
  }
  return os;
}

ostream &operator<<(ostream &os, conslist cons) {
  vector<cons1> &cc = cons.ls;
  REP(i, 0, cc.size()) {
    os << cc[i] << (i == cc.size() - 1 ? "" : " ");
  }
  return os;
}


ll l;

conslist lt(ll x, ll y) {
  vector<cons1> zero(0);
  if (x >= 0 && y >= 0) {
    return x < y ? vector<cons1>(1, cons1(0, inf)) : zero;
  }
  if (x < 0 && y >= 0) {
    if (x == -1) {
      return vector<cons1>(1, cons1(-inf, y - 1));
    }
    if (x == -2) {
      return vector<cons1>(1, cons1(l - y + 1, inf));
    }
  }
  if (x >= 0 && y < 0) {
    if (y == -1) {
      return vector<cons1>(1, cons1(x + 1, inf));
    }
    if (y == -2) {
      return vector<cons1>(1, cons1(-inf, l - x - 1));
    }
  }
  assert (x < 0 && y < 0);
  if (x == -1) { // ? < l - ?
    return vector<cons1>(1, cons1(-inf, (l - 1) / 2));
  }
  // l - ? < ?
  return vector<cons1>(1, cons1(l / 2 + 1, inf));
}
conslist neq(ll x, ll y) {
  return lt(x, y).cup(lt(y, x));
}

void check(ll x, ll y, ll z, conslist &cons) {
  conslist former = lt(x, y).intersection(lt(z, y)).intersection(neq(x, z));
  conslist latter = lt(y, x).intersection(lt(y, z)).intersection(neq(x, z));
  cons = cons.intersection(former.cup(latter));
  //cerr << "check(" << x << " " << y << " " << z << "):" << cons << endl;
}

int main(void){
  int t;
  cin >> t;
  while (t--) {
    cin >> l;
    vector<VL> a(3, VL(3, 0));
    int cnt = 0;
    REP(i, 0, 3) {
      REP(j, 0, 3) {
	cin >> a[i][j];
	if (a[i][j] == 0) {
	  cnt++;
	  a[i][j] = -cnt;
	}
      }
    }
    assert (cnt == 2);
    conslist cons(vector<cons1>(1, cons1(1, l - 1)));
    REP(i, 0, 3) {
      check(a[i][0], a[i][1], a[i][2], cons);
    }
    REP(i, 0, 3) {
      check(a[0][i], a[1][i], a[2][i], cons);
    }
    check(a[0][0], a[1][1], a[2][2], cons);
    check(a[0][2], a[1][1], a[2][0], cons);
    ll tot = 0;
    REP(i, 0, cons.ls.size()) {
      ll x = cons.ls[i].x;
      ll y = cons.ls[i].y;
      assert (x <= y);
      tot += y - x + 1;
    }
    cout << tot << endl;
  }
}
