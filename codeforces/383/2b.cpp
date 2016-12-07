#include <iostream>
#include <map>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;



int main(void){
  int n, x;
  cin >> n >> x;
  VI a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  map<int, int> freq;
  REP(i, 0, n) {
    if (freq.count(a[i]) == 0) {
      freq[a[i]] = 0;
    }
    freq[a[i]]++;
  }
  ll tot = 0;
  REP(i, 0, n) {
    tot += freq[x ^ a[i]] - (x == 0 ? 1 : 0);
  }
  cout << tot / 2 << endl;
}
