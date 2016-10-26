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



int main(void){
  string s;
  cin >> s;
  int a[26] = {};
  int n = s.length();
  REP(i, 0, n) {
    a[s[i] - 'a']++;
  }
  int prop = 0;
  REP(i, 0, 26) prop += a[i] % 2;
  if (prop <= 1) {
    cout << n << endl;
    return 0;
  }
  cout << (n - prop) / 2 / prop * 2 + 1 << endl;
  
}
