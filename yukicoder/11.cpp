#include <algorithm>
#include <iostream>
#include <set>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;


int main(void){
  ll w,h,n;
  cin >> w >> h >> n;
  set<int> suit, num;
  REP (i, 0, n) {
    int s,t;
    cin >> s >> t;
    suit.insert(s);
    num.insert(t);
  }
  cout << w * h - (w - suit.size()) * (h - num.size()) - n << endl;
}
