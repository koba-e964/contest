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
  cin >> n;
  if (n == 1) {
    cout << -1 << endl;
    return 0;
  }
  if (n % 2 == 0) {
    // 1 = 1/2 + 1/3 + 1/6
    cout << n << " " << n * 3 / 2 << " " << n * 3 << endl;
    return 0;
  }
  // 1/n = 1 / (n + 1) + 1 / n(n + 1)
  cout << n << " " << n + 1 << " " << n * (n + 1) << endl;
}
