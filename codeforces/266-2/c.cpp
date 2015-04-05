#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <set>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const double EPS=1e-9;

const int N=500100;

int n;
ll a[N];

ll cc[N];
template <class T> class BIT{
private:
	int n;
	T* ary;
public:
	BIT(int n) {
		if(n<=0)
			assert(0);
		while(n != (n & (-n))) {
			n += n & (-n);
		}
		this->n = n;
		ary = new T[n+1];
		for(int i=0; i<=n; i++) {
			ary[i] = 0;
		}
	}
	~BIT(void) {
		delete [] ary;
	}
	/**
	 * gets the sum in [1 .. ind]
	 * @param ind
	 * @return sum
	 */
	T accum(int ind) {
		T sum=0;
		while(ind>0) {
			sum += ary[ind];
			ind &= ind - 1;
		}
		return sum;
	}
	/**
	 * performs data[ind] += val;
	 */
	void add(int ind, T val) {
		if(ind <= 0) {
			assert(0);
		}
		while(ind <= n) {
			ary[ind] += val;
			ind += ind & (-ind);
		}
	}
	T size() {
		return ary[n];
	}
};
 
const int DEBUG = 0;
int main(void){
	cin>>n;
	REP(i,0,n) cin >> a[i];
	ll sum = 0;
	REP(i,0,n) {
		sum += a[i];
		cc[i]=sum;
	}
	if(cc[n-1] % 3) {
		cout << 0 << endl;
		return 0;
	}
	ll t = cc[n-1] / 3;
	BIT<ll> bit(n+1);
	REP(i,0,n) {
		if(cc[i] == t) {
			bit.add(i+1,1);
		}
	}
	if(DEBUG) {
		REP(i,0,n) {
			cout << i << ":" << cc[i] << endl;
		}
	}
	ll tot=0;
	REP(i,1,n-1) {
		if(cc[i] == 2 * t) {
			tot += bit.accum(i);
		}
	}
	cout << tot << endl;
	
}
