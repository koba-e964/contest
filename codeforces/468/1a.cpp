#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int N = 100100;

int depc[N];

VI child[N];
int p[N];

void dfs(int v, int d) {
  depc[d]++;
  REP(i, 0, child[v].size()) {
    int w = child[v][i];
    dfs(w, d + 1);
  }
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  REP(i, 1, n) {
    cin >> p[i];
    p[i]--;
    child[p[i]].push_back(i);
  }
  dfs(0, 0);
  int c = 0;
  REP(i, 0, N) {
    c += depc[i] % 2;
  }
  cout << c << "\n";
}
