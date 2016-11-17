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
const int DEBUG = 0;

const int N = 200100;

int n;
int x[N];
VI edges[N];
VI rev_e[N];

/*
 * 0: white, 1: black, 2: gray
 */
bool retrograde(VI &col) {
  // retrograde analysis
  queue<int> t;
  REP(i, 0, 2 * n) {
    if (col[i] == 2) {
      t.push(i);
    }
  }
  while (not t.empty()) { // vulnerable to inf loop
    int v = t.front(); t.pop();
    bool mod = false;
    if (col[v] == 2) { // try updating
      int wh = 0, gr = 0, bl = 0;
      REP(i, 0, edges[v].size()) {
	int w = edges[v][i];
	if (col[w] == 0) wh++;
	if (col[w] == 1) bl++;
	if (col[w] == 2) gr++;
      }
      if (wh >= 1) {
	col[v] = 1; // win
	mod = true;
      }
      if (wh == 0 && gr == 0) {
	col[v] = 0; // lose
	mod = true;
      }
    }
    if (mod) {
      REP(i, 0, rev_e[v].size()) {
	int w = rev_e[v][i];
	t.push(w);
      }
    }
  }
  if (DEBUG) {
    REP(i, 0, 2 * n) {
      cerr << col[i] << " ";
    }
    cerr << endl;
  }
  return col[0] == 1;
}

int main(void){
  int m;
  cin >> n >> m;
  REP(i, 0, n) {
    cin >> x[i];
  }
  REP(i, 0, m) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    // [0, n): for Player 1, [n, 2n): for Player 2
    edges[a].push_back(b + n);
    edges[a + n].push_back(b);
    rev_e[b].push_back(a + n);
    rev_e[n + b].push_back(a);
  }
  int lo = -1;
  int hi = 1e9 + 1;
  while (hi - lo > 1) {
    int mid = (hi + lo) / 2;
    VI col(2 * n, 2);
    REP(i, 0, n) {
      if (x[i] >= mid) {
	col[i] = 1;
      } else {
	col[n + i] = 1;
      }
    }
    if (DEBUG) {
      cerr << "mid = " << mid << endl;
    }
    if (retrograde(col)) {
      lo = mid;
    } else {
      hi = mid;
    }
  }
  cout << lo << endl;
}
