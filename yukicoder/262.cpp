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
#include <unordered_map>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<ll, ll> PL;
const ll mod = 1e9 + 7;

const ll B = 1 << 25;

unordered_map<ll, PL> memo;

PL big_step(int bias, ll c) {
  assert (c < 41);
  ll code = bias * 64 + c;
  if (memo.count(code)) {
    return memo[code];
  }
  int cnt = 0;
  while (c < B) {
    c += __builtin_popcountll(c) + bias;
    cnt++;
  }
  memo[code] = PL(cnt, c);
  return PL(cnt, c);
}

int main(void){
  ll n;
  cin >> n;
  ll c = 1;
  ll cnt = 0;
  while (c < n) {
    PL result = big_step(__builtin_popcountll(c / B), c % B);
    ll newc = c / B * B + result.second;
    if (newc > n) {
      break;
    }
    cnt += result.first;
    c = newc;
  }
  while (c < n) {
    c += __builtin_popcountll(c);
    cnt++;
  }
  cout << (c == n ? cnt + 1 : -1) << endl;
}
