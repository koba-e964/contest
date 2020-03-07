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


double solve(ll x, double p) {
  return (x + 1) / 2 / p;
}

int main(void){
  ll x;
  int p;
  cin >> x >> p;
  if (p == 100) {
    cout << (x + 1) / 2 << endl;
    return 0;
  }
  printf("%.15f\n", solve(x, p / 100.0));
}
