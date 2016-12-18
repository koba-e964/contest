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
  string s;
  cin >> n >> s;
  vector<char> c(n, '\0');
  REP(i, 0, n / 2) {
    int idx = n - i * 2 - 2;
    c[i] = s[idx];
    c[n - 1 - i] = s[idx + 1];
  }
  if (n % 2 == 1) {
    c[n / 2] = s[0];
  }
  string t;
  REP(i, 0, n) {
    t += c[i];
  }
  cout << t << endl;
}
