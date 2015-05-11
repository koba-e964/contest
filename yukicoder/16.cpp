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
typedef pair<int, int> PI;
const double EPS=1e-9;

const int mod = 1e6 +3;
int calc(int x, int e) {
  ll sum = 1;
  ll cur = x;
  while (e > 0) {
    if (e % 2) {
      sum *= cur;
      sum %= mod;
    }
    cur *= cur;
    cur %= mod;
    e /= 2;
  }
  return (int)sum;
}




int main(void){
  int x, n;
  cin >> x >> n;
  int s = 0;
  REP(i, 0, n) {
    int t; cin >> t;
    s += calc(x, t);
    s %= mod;
  }
  cout << s << endl;
}
