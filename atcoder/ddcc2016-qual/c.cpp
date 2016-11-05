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



int main(void){
  int n;
  ll k;
  cin >> n >> k;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
    a[i] = __gcd(a[i], k);
  }
  sort(a.begin(), a.end());
  VL fact;
  REP(i, 1, sqrt(k) + 1) {
    if (k % i == 0) {
      fact.push_back(i);
      if (k != i * i) {
	fact.push_back(k / i);
      }
    }
  }
  sort(fact.begin(), fact.end()); // divisors of k
  map<int, int> rev_fact;
  vector<int> nums(fact.size(), 0);
  vector<int> acc(fact.size(), 0);
  int fs = fact.size();
  REP(i, 0, fact.size()) {
    ll f = fact[i];
    rev_fact[f] = i;
  }
  REP(i, 0, n) {
    nums[rev_fact[a[i]]]++;
  }
  REP(i, 0, fs) {
    REP(j, i, fs) {
      if (fact[j] % fact[i] == 0) {
        acc[i] += nums[j];
      }
    }
  }
  ll cnt = 0;
  REP(i, 0, fs) {
    cnt += ll(acc[i]) * ll(nums[fs - 1 - i]);
  }
  REP(i, 0, n) {
    if (a[i] * a[i] % k == 0) cnt--;
  }
  cout << cnt / 2 << endl;
}
