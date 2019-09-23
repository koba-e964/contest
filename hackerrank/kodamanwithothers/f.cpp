#include <stdio.h>
#include <iostream>
#include <vector>
#include <algorithm>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long ll;

int main(void){
  int n, k;
  cin >> n >> k;
  vector<ll> d;
  ll sum = 0;
  REP(i, 0, n) {
    ll a, b;
    cin >> a >> b;
    sum += a;
    d.push_back(a - b);
  }
  sort(d.rbegin(), d.rend());
  REP(i, 0, k) {
    sum -= d[i];
  }
  cout << sum << endl;
}
