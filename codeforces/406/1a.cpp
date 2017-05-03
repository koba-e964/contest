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



int main(void){
  int n;
  cin >> n;
  int k1;
  cin >> k1;
  VI s1(k1);
  REP(i, 0, k1) {
    cin >> s1[i];
  }
  int k2;
  cin >> k2;
  VI s2(k2);
  REP(i, 0, k2) {
    cin >> s2[i];
  }
  // Backward analysis
  vector<vector<bool> > rev(2 * n - 1, vector<bool>(2 * n - 1, false));
  // 0: 0 (black hole)
  // [1, n): player 1
  // [n, 2 * n - 1): player 2
  REP(i, 1, n) {
    REP(j, 0, k1) {
      int p = i + s1[j];
      if (p == n) {
	p = 0;
      } else if (p >= n + 1) {
	p -= 1;
      } else {
	p += n - 1;
      }
      rev[p][i] = true;
    }
    REP(j, 0, k2) {
      int p = i + s2[j];
      p = p >= n + 1 ? p - n : (p == n ? 0 : p);
      rev[p][n - 1 + i] = true;
    }
  }
  queue<short> que;
  que.push(-1); // [1, 2n]: +1, [-2n, -1]: -1
  VI res(2 * n - 1, 0); // 0: Loop, 1: Win, 2: Lose
  vector<bool> vis(2 * n - 1);
  VI nums(2 * n - 1);
  nums[0] = -1;
  while (not que.empty()) {
    int top = que.front(); que.pop();
    int t = abs(top) - 1;
    int win = top > 0 ? 1 : -1;
    if (0) {
      cerr << endl << "t=" << t << endl;
      cerr << "res:";
      REP(i, 0, 2 * n - 1) {
	cerr << " " << res[i];
      }
      cerr << endl;
      cerr << "vis:";
      REP(i, 0, 2 * n - 1) {
	cerr << " " << vis[i];
      }
      cerr << endl;
      cerr << "nums:";
      REP(i, 0, 2 * n - 1) {
	cerr << " " << nums[i];
      }
      cerr << endl;
    }
    if (win == -1) {
      nums[t] += 1;
      int ne = t == 0 ? 0 : t < n ? k1 : k2;
      if (nums[t] == ne) {
	assert (res[t] == 0);
        res[t] = -1;
      }
    }
    if (win == 1) {
      res[t] = 1;
    }
    if (res[t] != 0) {
      if (vis[t]) {
	continue;
      }
      REP(i, 0, 2 * n - 1) {
	if (not rev[t][i]) { continue; }
	if (not vis[i]) {
	  que.push(-res[t] * (i + 1));
	}
      }
      vis[t] = true;
    }
  }
  const char* dat[3] = {"Lose", "Loop", "Win"};
  REP(i, 0, 2 * (n - 1)) {
    cout << dat[res[i + 1] + 1] << (i % (n - 1) == n - 2 ? "\n" : " ");
  }
}
