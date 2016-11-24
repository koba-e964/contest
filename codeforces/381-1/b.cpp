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
typedef pair<int, ll> PIL;
const ll mod = 1e9 + 7;

const int N = 200100;
const int B = 18;
ll a[N];
int p[N];
ll w[N];
ll dist[N];
ll diff[N];
vector<PIL> child[N];
int spt[B][N];
int acc[N] = {};

void dfs(int v, ll d) {
  dist[v] = d;
  diff[v] = dist[v] - a[v];
  REP(i, 0, child[v].size()) {
    PIL e = child[v][i];
    dfs(e.first, d + e.second);
  }
}

int postproc(int v) {
  int sum = acc[v];
  REP(i, 0, child[v].size()) {
    PIL e = child[v][i];
    sum += postproc(e.first);
  }
  acc[v] = sum;
  return sum;
}

int main(void){
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  REP(i, 0, n) {
    cin >> a[i];
  }
  REP(i, 0, n - 1) {
    cin >> p[i + 1] >> w[i + 1];
    p[i + 1]--;
    child[p[i + 1]].push_back(PIL(i + 1, w[i + 1]));
  }
  p[0] = -1;
  w[0] = 0;
  dfs(0, 0);
  // Need to find |{u | u is a descendant of v and diff[u] <= dist[v]}| for every v
  REP(i, 0, n) {
    spt[0][i] = p[i];
  }
  REP(b, 1, B) {
    REP(i, 0, n) {
      int mp = spt[b - 1][i];
      spt[b][i] = mp == -1 ? -1 : spt[b - 1][mp];
    }
  }
  REP(i, 0, n) {
    // Find the highest ancestor v of i s.t. dist[v] >= diff[i]. i itself satisfies it.
    int idx = 0;
    int cur = i;
    for (int b = B - 1; b >= 0; --b) {
      int mp = spt[b][cur];
      if (mp == -1 || dist[mp] < diff[i]) {
	idx += 0;
      } else {
	idx += 1 << b;
	cur = mp;
      }
    }
    acc[i] += 1;
    if (cur != 0) {
      acc[p[cur]] -= 1;
    }
  }
  postproc(0);
  REP(i, 0, n) {
    cout <<  acc[i] - 1 << (i == n - 1 ? '\n' : ' ');
  }
}
