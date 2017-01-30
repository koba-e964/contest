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


// NAF-based
VI solve(int n) {
  VI res;
  int cur = 1;
  VI pos;
  VI neg;
  int oldn = n;
  while (n > 0) {
    if (n % 2 == 1) {
      int z = 2 - (n % 4);
      n -= z;
      if (z > 0) {
	if (cur > oldn) {
	  pos.push_back(cur / 2);
	  pos.push_back(cur / 2);
	} else {
	  pos.push_back(cur);
	}
      } else {
	neg.push_back(-cur);
      }
    }
    n /= 2;
    cur *= 2;
  }
  cur = 0;
  int p = 0;
  if (pos.size() > 0 && neg.size() > 0) {
    int bar = pos[pos.size() - 1];
    int foo = neg[neg.size() - 1];
    if (bar == -2 * foo) {
      pos[pos.size() - 1] = bar / 2;
      neg.erase(neg.end() - 1);
    }
  }
  REP(i, 0, pos.size()) {
    cur += pos[i];
    res.push_back(pos[i]);
    while (p < neg.size()) {
      if (neg[p] + cur >= 0) {
	res.push_back(neg[p]);
	cur += neg[p];
	p++;
      } else {
	break;
      }
    }
  }
  assert (p == neg.size());
  return res;
}

int main(void){
  int n;
  cin >> n;
  VI ts = solve(n);
  int tot = 0;
  int cur = 0;
  REP(i, 0, ts.size()) {
    int v = ts[i];
    if (v >= 1) {
      int c;
      cout << "? " << cur << " " << cur + v << endl;
      cin >> c;
      tot += c;
    } else {
      int c;
      cout << "? " << cur + v << " " << cur << endl;
      cin >> c;
      tot -= c;
    }
    cur += v;
  }
  cout << "! " << tot << endl;
}
