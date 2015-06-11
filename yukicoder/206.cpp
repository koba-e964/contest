#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef unsigned long long int ll;

const int N = 100010;
const int S = 64;
const int M = (N + S - 1) / S;
int l, m, n;
ll a[M], b[M];
ll u[S][M];
int q;
int main(void){
  cin >> l >> m >> n;
  REP(i, 0, l) {
    int t;
    cin >> t;
    a[t / S] |= 1LL << (t % S);
  }
  REP(i, 0, m) {
    int t;
    cin >> t;
    b[t / S] |= 1LL << (t % S);
  }
  REP(i, 0, S) {
    u[i][0] = b[0] << i;
    REP(j, 1, M) {
      u[i][j] = b[j] << i | (i == 0 ? 0 : b[j - 1] >> (S - i));
    }
  }
  cin >> q;
  REP(v, 0, q) {
    int cnt = 0;
    REP(i, 0, M - v / S) {
      cnt += __builtin_popcountll(a[i + v / S] & u[v % S][i]);
    }
    cout << cnt << endl;
  }
}
