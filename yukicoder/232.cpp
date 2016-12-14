#include <cassert>
#include <iostream>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;


void fail() {
  cout << "NO\n";
  exit(0);
}

int main(void){
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t, a, b;
  cin >> t >> b >> a;
  if (t < max(a, b)) {
    fail();
  }
  if (t == 1 && a + b == 0) {
    fail();
  }
  VI seq;
  REP(i, 0, min(a, b)) {
    seq.push_back(3);
  }
  REP(i, min(a, b), a) {
    seq.push_back(1);
  }
  REP(i, min(a, b), b) {
    seq.push_back(2);
  }
  assert (seq.size() == max(a, b));
  if (t - max(a, b) != 1) {
    if ((t - max(a, b)) % 2 == 0) {
      REP(i, 0, (t - max(a, b)) / 2) {
	seq.push_back(1); // >
	seq.push_back(4); // <
      }
    } else {
      seq.push_back(3); // >^
      seq.push_back(4); // <
      seq.push_back(5); // v
      REP(i, 1, (t - max(a, b) - 1) / 2) {
	seq.push_back(1);
	seq.push_back(4);
      }
    }
  } else {
    int &last = seq[seq.size() - 1];
    if (last == 1) {
      last = 3;
      seq.push_back(5);
    } else if (last == 2) {
      last = 3;
      seq.push_back(4);
    } else {
      assert (last == 3);
      last = 1;
      seq.push_back(2);
    }
  }
  assert (seq.size() == t);
  cout << "YES\n";
  REP(i, 0, t) {
    int cmd = seq[t - 1 - i];
    string s = "test";
    if (cmd == 1) {
      s = ">";
      a--;
    }
    if (cmd == 2) {
      s = "^";
      b--;
    }
    if (cmd == 3) {
      s = ">^";
      a--, b--;
    }
    if (cmd == 4) {
      s = "<";
      a++;
    }
    if (cmd == 5) {
      s = "v";
      b++;
    }
    assert (s != "test");
    cout << s << "\n";
  }
  assert (a == 0 && b == 0);
}
