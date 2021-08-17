#include <algorithm>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
const int N = 2010;

const int DEBUG = 0;
VI graph[N];
int parent[N];
int desc[N];

ll dp[N][N];
const ll mod = 1e9+7;
void rec_p(int n, int p) {
  parent[n] = -2;
  int s = 0;
  REP(i, 0, graph[n].size()) {
    int v = graph[n][i];
    if (parent[v] != -2) {
      rec_p(v, n);
      s += desc[v] + 1;
    }
  }
  parent[n] = p;
  desc[n] = s;
}
// [0,d]
void poly_mul(ll aa[N], const ll bb[N], int da, int db) {
  ll cc[N] = {0};
  REP(i, 0, da + 1) {
    REP(j, 0, db + 1) {
      cc[i + j] += aa[i] * bb[j];
      cc[i + j] %= mod;
    }
  }
  REP(i, 0, da + db + 1) {
    if (DEBUG) {
      cout << cc[i] << "x^" << i << "+";
    }

    aa[i] = cc[i];
  }
  if (DEBUG) {
    cout << endl;
  }
}

void rec(int v) {
  dp[v][0] = 1;
  int d = 0;
  REP(i, 0, graph[v].size()) {
    int u = graph[v][i];
    if (u == parent[v]) {
      continue;
    }
    rec(u);
    poly_mul(dp[v], dp[u], d, desc[u] + 1);
    d += desc[u] + 1;
  }
  dp[v][desc[v] + 1] = 1;
}

int main(void){
  int n, k;
  cin >> n >> k;
  REP (i, 0, n - 1) {
    int a, b;
    cin >> a >> b;
    graph[a].push_back(b);
    graph[b].push_back(a);
  }
  rec_p(0, -1);
  if (DEBUG) {
    REP(i, 0, n) {
      cout << "p[" << i << "]=" << parent[i] << endl;
    }
  }
  REP(i,0,N) {
    fill_n(dp[i], N, 0);
    dp[i][0] = 1;
  }
  rec(0);
  cout << dp[0][k] << endl;
}
