#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;

const int N = 100100;
VI edges[N];

ll dp[N];

void dfs(int v, int par) {
  VL ch;
  ll orl = 0;
  for (auto w: edges[v]) {
    if (w == par) { continue; }
    dfs(w, v);
    ch.push_back(dp[w]);
    orl |= dp[w];
  }
  vector<int> hb(64);
  REP(b, 0, 64) {
    int cnt = 0;
    for (ll t: ch) {
      if (t & (1LL << b)) {
	cnt += 1;
      }
    }
    hb[b] = cnt;
  }
  int c = 0;
  REP(b, 0, 64) {
    if (hb[b] >= 2) {
      c = b + 1;
    }
  }
  int t = -1;
  REP(b, c, 64) {
    if (hb[b] == 0) {
      t = b;
      break;
    }
  }
  ll mask = 1LL << t;
  dp[v] = (orl & ~(mask - 1)) | mask;
}

// This was implemented after the author read the editorial.
int main(void){
  int n;
  cin >> n;
  REP(i, 0, n - 1) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    edges[a].push_back(b);
    edges[b].push_back(a);
  }
  dfs(0, -1);
  int b = 0;
  REP(i, 0, 64) {
    if (dp[0] & 1LL << i) {
      b = i;
    }
  }
  cout << b << endl;
}
