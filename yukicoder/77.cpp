#include <bits/stdc++.h>
#include <algorithm>
using namespace std;
#define rep(i,s,n) for(int i=(s); i<(int)(n); ++i)
int n;
int a[1000];
int main(){
  cin >> n;
  int s = 0;
  rep(i, 0, n) {
    cin >> a[i];
    s += a[i];
  }
  int mi = 0x3fffffff;
  for(int k = 1; k * k <= s; k++){
    int d = 0;
    rep(j, 0, k - 1) {
      int t = a[j] - j - 1;
      d += max(0, t);
    }
    rep(j, 0, k) {
      int t = a[j + k - 1] - k + j;
      d += max(0, t);
    }
    rep(j, 2 * k - 1, n) {
      d += a[j];
    }
    mi = min(mi, d);
  }
  cout << mi << endl;
}
