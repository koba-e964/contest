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

void fail() {
  cout << -1 << endl;
  exit(0);
}

int main(void){
  int n, k;
  cin >> n >> k;
  vector<int> a(k);
  REP(i, 0, k) {
    cin >> a[i];
    a[i]--;
  }
  vector<string> s(n);
  REP(i, 0, n) {
    cin >> s[i];
  }
  if (n == k) { // corner
    cout << "" << endl;
    return 0;
  }
  // find the longest prefix of s[a[i]]
  int pos = 0;
  while (true) {
    bool ok = true;
    set<char> p;
    REP(i, 0, k) {
      if ((int) s[a[i]].length() <= pos) {
	ok = false;
	break;
      }
      p.insert(s[a[i]][pos]);
    }
    if (p.size() != 1) {
      ok = false;
    }
    if (ok) {
      pos += 1;
    } else {
      break;
    }
  }
  if (pos == 0) {
    fail();
  }
  string common = s[a[0]].substr(0, pos);
  vector<bool> subset(n, false);
  REP(i, 0, k) {
    subset[a[i]] = true;
  }
  int pos2 = 0;
  int cnt = k;
  while (pos2 < pos) {
    REP(i, 0, n) {
      if (subset[i]) { continue; }
      if ((int) s[i].length() <= pos2) {
        subset[i] = true;
	cnt += 1;
	continue;
      }
      if (s[i][pos2] != common[pos2]) {
        subset[i] = true;
	cnt += 1;
	continue;
      }
    }
    if (cnt >= n) {
      break;
    }
    pos2 += 1;
  }
  if (pos2 == pos) fail();
  cout << common.substr(0, pos2 + 1) << endl;
}
