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

void fail(void) {
  cout << 0 << endl;
  exit(0);
}

int main(void){
  ll s, x;
  cin >> s >> x;
  if (s < x) {
    fail();
  }
  if (s % 2 != x % 2) {
    fail();
  }
  ll a = (s - x) / 2;
  if (a & x) {
    fail();
  }
  ll tmp = 1LL << __builtin_popcountll(x);
  if (a == 0) {
    tmp -= 2;
  }
  cout << tmp << endl;
}
