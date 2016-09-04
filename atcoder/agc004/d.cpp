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

int n;
ll k;
const int N = 100100;
int a[N];
VI chi[N];
int cnt = 0;
int d[N];
void dfs(int v, int qq) {
  d[v] = qq;
  REP(i, 0, chi[v].size()) {
    int c = chi[v][i];
    dfs(c, qq + 1);
  }
}
int marked[N];
void mark(int v) {
  if (marked[v]) {
    return;
  }
  marked[v] = 1;
  REP(i, 0, chi[v].size()) {
    int c = chi[v][i];
    mark(c);
  }
}

const int B = 20;
int lbt[N][B];
void lbt_init(void) {
  REP(i, 0, n) {
    lbt[i][0] = a[i];
  }
  REP(b, 1, B) {
    REP(i, 0, n) {
      lbt[i][b] = lbt[lbt[i][b - 1]][b - 1];
    }
  }
}

int get_anc(int v, int l) {
  REP(i, 0, B) {
    if (l & (1 << i)) {
      v = lbt[v][i];
    }
  }
  return v;
}

int main(void){
  cin >> n >> k;
  REP(i, 0, n) {
    cin >> a[i];
    a[i]--;
  }
  if (a[0] != 0) {
    a[0] = 0;
    cnt = 1;
  }
  if (k == 1) {
    REP(i, 0, n) {
      if (a[i] != 0) {
	a[i] = 0;
	cnt++;
      }
    }
    cout << cnt << endl;
    return 0;
  }
  if (k >= n - 1) {
    cout << cnt << endl;
    return 0;
  }
  REP(i, 1, n) {
    chi[a[i]].push_back(i);
  }
  dfs(0, 0);
  vector<PI> que;
  REP(i, 0, n) {
    que.push_back(PI(d[i], i));
  }
  sort(que.rbegin(), que.rend());
  lbt_init();
  REP(i, 0, que.size()) {
    PI t = que[i];
    //cerr << t.first << " " << t.second << endl;
    if (!marked[t.second] && t.first >= k + 1) {
      // mark k - 1 up vertex TODO
      /*
      int cur = t.second;
      REP(i, 0, k - 1) {
	cur = a[cur];
      }
      */
      int up = get_anc(t.second, k - 1);
      //mark
      mark(up);
      //      cerr << "mark:" << up << endl;
      cnt++;
    }
  }
  cout << cnt << endl;
}
