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
typedef pair<int, int> PI;

const int N = 100100;
int rem[N];
vector<int> res;
int sub[N];


int main(void){
  int k, n;
  string t;
  cin >> k >> t;
  n = t.size();
  int acc = 0;
  REP(i, 0, n) {
    if (t[i] == 's') {
      acc++;
    } else {
      if (acc > 0) {
	res.push_back(-acc);
	acc = 0;
      }
      res.push_back(t[i]);
    }
  }
  if (acc > 0) {
    res.push_back(-acc);
    acc = 0;
  }
  rem[res.size()] = 0;
  for (int i = res.size() - 1; i >= 0; --i) {
    rem[i] = rem[i + 1] + (res[i] < 0 ? -res[i] : 0);
  }
  int r = k;
  REP(i, 0, res.size()) {
    assert(0 <= r && r <= rem[i]);
    if (res[i] >= 0) {
      continue;
    }
    int flag = i == res.size() - 1 || res[i + 1] < 's'; // 1 : if replaced, it produces lexicographically less string.
    int lo = max(0, r - rem[i + 1]);
    int hi = min(-res[i], r);
    if (flag) {
      sub[i] = hi;
      r -= hi;
      continue;
    }
    sub[i] = lo;
    r -= lo;
  }
  REP(i, 0, res.size()) {
    if (res[i] >= 0) {
      cout << (char)res[i];
    } else {
      REP(j, 0, -res[i] - sub[i]) {
	cout << 's';
      }
    }
  }
  cout << endl;
}
