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
  int n, p;
  cin >> n >> p;
  VI a(n);
  bool odd = false;
  REP(i, 0, n) {
    cin >> a[i];
    a[i] %= 2;
    if (a[i] == 1) {
      odd = true;
    }
  }
  if (odd) {
    cout << (1LL << n - 1) << endl;
  } else {
    cout << (p ? 0 : 1LL << n) << endl;
  }
  
}
