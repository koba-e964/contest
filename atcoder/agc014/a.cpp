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
  ll a, b, c;
  cin >> a >> b >> c;
  if (a % 2 == 1 || b % 2 == 1 || c % 2 == 1) {
    cout << 0 << endl;
    return 0;
  }
  if (a == b && b == c) {
    cout << -1 << endl;
    return 0;
  }
  ll d = abs(a - b);
  ll e = abs(b - c);
  int k = 0;
  while (true) {
    if ((d & 1LL << k) || (e & 1LL << k)) {
      break;
    }
    k++;
  }
  cout << k << endl;
}
