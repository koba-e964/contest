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

ll odd_palin(int v) {
  ll w = v / 10;
  ll sum = 0;
  ll cur = 1;
  while (w > 0) {
    sum *= 10;
    sum += w % 10;
    cur *= 10;
    w /= 10;
  }
  return sum + cur * v;
}
ll even_palin(int v) {
  ll w = v;
  ll sum = 0;
  ll cur = 1;
  while (w > 0) {
    sum *= 10;
    sum += w % 10;
    cur *= 10;
    w /= 10;
  }
  return sum + cur * v;
}

ll calc_palin(ll n) {
  int cnt = 0;
  REP(i, 1, 100000) {
    if (odd_palin(i) <= n) { cnt += 1; }
    if (even_palin(i) <= n) { cnt += 1; }
  }
  return cnt;
}

int main(void){
  ll n;
  cin >> n;
  cout << calc_palin(n / 1000000001) << endl;
}
