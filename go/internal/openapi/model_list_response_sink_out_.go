/*
 * Svix API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * API version: 1.1.1
 */

// Code generated by OpenAPI Generator (https://openapi-generator.tech); DO NOT EDIT.

package openapi

import (
	"encoding/json"
)

// ListResponseSinkOut struct for ListResponseSinkOut
type ListResponseSinkOut struct {
	Data []SinkOut `json:"data"`
	Done bool `json:"done"`
	Iterator NullableString `json:"iterator,omitempty"`
	PrevIterator NullableString `json:"prevIterator,omitempty"`
}

// NewListResponseSinkOut instantiates a new ListResponseSinkOut object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewListResponseSinkOut(data []SinkOut, done bool) *ListResponseSinkOut {
	this := ListResponseSinkOut{}
	this.Data = data
	this.Done = done
	return &this
}

// NewListResponseSinkOutWithDefaults instantiates a new ListResponseSinkOut object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewListResponseSinkOutWithDefaults() *ListResponseSinkOut {
	this := ListResponseSinkOut{}
	return &this
}

// GetData returns the Data field value
func (o *ListResponseSinkOut) GetData() []SinkOut {
	if o == nil {
		var ret []SinkOut
		return ret
	}

	return o.Data
}

// GetDataOk returns a tuple with the Data field value
// and a boolean to check if the value has been set.
func (o *ListResponseSinkOut) GetDataOk() (*[]SinkOut, bool) {
	if o == nil  {
		return nil, false
	}
	return &o.Data, true
}

// SetData sets field value
func (o *ListResponseSinkOut) SetData(v []SinkOut) {
	o.Data = v
}

// GetDone returns the Done field value
func (o *ListResponseSinkOut) GetDone() bool {
	if o == nil {
		var ret bool
		return ret
	}

	return o.Done
}

// GetDoneOk returns a tuple with the Done field value
// and a boolean to check if the value has been set.
func (o *ListResponseSinkOut) GetDoneOk() (*bool, bool) {
	if o == nil  {
		return nil, false
	}
	return &o.Done, true
}

// SetDone sets field value
func (o *ListResponseSinkOut) SetDone(v bool) {
	o.Done = v
}

// GetIterator returns the Iterator field value if set, zero value otherwise (both if not set or set to explicit null).
func (o *ListResponseSinkOut) GetIterator() string {
	if o == nil || o.Iterator.Get() == nil {
		var ret string
		return ret
	}
	return *o.Iterator.Get()
}

// GetIteratorOk returns a tuple with the Iterator field value if set, nil otherwise
// and a boolean to check if the value has been set.
// NOTE: If the value is an explicit nil, `nil, true` will be returned
func (o *ListResponseSinkOut) GetIteratorOk() (*string, bool) {
	if o == nil  {
		return nil, false
	}
	return o.Iterator.Get(), o.Iterator.IsSet()
}

// HasIterator returns a boolean if a field has been set.
func (o *ListResponseSinkOut) HasIterator() bool {
	if o != nil && o.Iterator.IsSet() {
		return true
	}

	return false
}

// SetIterator gets a reference to the given NullableString and assigns it to the Iterator field.
func (o *ListResponseSinkOut) SetIterator(v string) {
	o.Iterator.Set(&v)
}
// SetIteratorNil sets the value for Iterator to be an explicit nil
func (o *ListResponseSinkOut) SetIteratorNil() {
	o.Iterator.Set(nil)
}

// UnsetIterator ensures that no value is present for Iterator, not even an explicit nil
func (o *ListResponseSinkOut) UnsetIterator() {
	o.Iterator.Unset()
}

// GetPrevIterator returns the PrevIterator field value if set, zero value otherwise (both if not set or set to explicit null).
func (o *ListResponseSinkOut) GetPrevIterator() string {
	if o == nil || o.PrevIterator.Get() == nil {
		var ret string
		return ret
	}
	return *o.PrevIterator.Get()
}

// GetPrevIteratorOk returns a tuple with the PrevIterator field value if set, nil otherwise
// and a boolean to check if the value has been set.
// NOTE: If the value is an explicit nil, `nil, true` will be returned
func (o *ListResponseSinkOut) GetPrevIteratorOk() (*string, bool) {
	if o == nil  {
		return nil, false
	}
	return o.PrevIterator.Get(), o.PrevIterator.IsSet()
}

// HasPrevIterator returns a boolean if a field has been set.
func (o *ListResponseSinkOut) HasPrevIterator() bool {
	if o != nil && o.PrevIterator.IsSet() {
		return true
	}

	return false
}

// SetPrevIterator gets a reference to the given NullableString and assigns it to the PrevIterator field.
func (o *ListResponseSinkOut) SetPrevIterator(v string) {
	o.PrevIterator.Set(&v)
}
// SetPrevIteratorNil sets the value for PrevIterator to be an explicit nil
func (o *ListResponseSinkOut) SetPrevIteratorNil() {
	o.PrevIterator.Set(nil)
}

// UnsetPrevIterator ensures that no value is present for PrevIterator, not even an explicit nil
func (o *ListResponseSinkOut) UnsetPrevIterator() {
	o.PrevIterator.Unset()
}

func (o ListResponseSinkOut) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if true {
		toSerialize["data"] = o.Data
	}
	if true {
		toSerialize["done"] = o.Done
	}
	if o.Iterator.IsSet() {
		toSerialize["iterator"] = o.Iterator.Get()
	}
	if o.PrevIterator.IsSet() {
		toSerialize["prevIterator"] = o.PrevIterator.Get()
	}
	return json.Marshal(toSerialize)
}

type NullableListResponseSinkOut struct {
	value *ListResponseSinkOut
	isSet bool
}

func (v NullableListResponseSinkOut) Get() *ListResponseSinkOut {
	return v.value
}

func (v *NullableListResponseSinkOut) Set(val *ListResponseSinkOut) {
	v.value = val
	v.isSet = true
}

func (v NullableListResponseSinkOut) IsSet() bool {
	return v.isSet
}

func (v *NullableListResponseSinkOut) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableListResponseSinkOut(val *ListResponseSinkOut) *NullableListResponseSinkOut {
	return &NullableListResponseSinkOut{value: val, isSet: true}
}

func (v NullableListResponseSinkOut) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableListResponseSinkOut) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}

