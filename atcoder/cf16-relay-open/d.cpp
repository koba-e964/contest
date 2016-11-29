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
  int a, b, c;
  cin >> a >> b >> c;
  int tmp = 4 * c - 2 * a - b;
  cout << a << " " << b << " " << 3 * c - a - b << endl;
  cout << tmp << " " << c << " " << 2 * c - tmp  << endl;
  cout << a + b - c << " " << 2 * c - b << " " << 2 * c - a << endl;
}
