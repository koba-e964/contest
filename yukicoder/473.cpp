#include <algorithm>
#include <iostream>
#include <unordered_map>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;

const int F = 1500;
const int N = 31;
unordered_map<int, ll> memo[F][N];

ll solve(const VI &fact, int k, int rem, int x) {
  if (x == 1) {
    return rem <= 0 ? 1 : 0;
  }
  if (k >= fact.size()) {
    return 0;
  }
  if (x < fact[k]) {
    return 0;
  }
  if (rem <= 0) {
    return 0;
  }
  ll f = fact[k];
  if (rem == 1) {
    return x >= f ? 1 : 0;
  }
  if (memo[k][rem].count(x)) {
    return memo[k][rem][x];
  }
  ll ret = 0;
  if (x % f == 0) {
    ret += solve(fact, k, rem - 1, x / f);
  }
  ret += solve(fact, k + 1, rem, x);
  return memo[k][rem][x] = ret;
}

int main(void){
  int n;
  int x;
  cin >> n >> x;
  x += 1;
  if (n >= 30) {
    cout << 0 << endl;
    return 0;
  }
  VI fact;
  for (int i = 1; i * i <= x; ++i) {
    if (x % i == 0) {
      if (i > 1) {
	fact.push_back(i);
      }
      if (x != i * i) {
	fact.push_back(x / i);
      }
    }
  }
  sort(fact.begin(), fact.end());
  cout << solve(fact, 0, n, x) << endl;
}
