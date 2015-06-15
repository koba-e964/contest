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
  int l;
  cin >> l;
  l /= 4;
  int k = sqrt(l) + 2;
  ll cnt = 0;
  REP(i, 1, k) {
    REP(j, i + 1, k) {
      if (__gcd(i, j) >= 2) {
	continue;
      }
      if ((i - j) % 2 == 0) {
	continue;
      }
      int a = i * j * 2;
      int b = j * j - i * i;
      int c = i * i + j * j;
      if (a + b + c <= l) {
	cnt++;
      }
    }
  }
  cout << cnt << endl;
}
