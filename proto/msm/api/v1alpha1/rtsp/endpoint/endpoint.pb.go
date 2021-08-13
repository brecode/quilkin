// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.25.0
// 	protoc        v3.17.3
// source: endpoint/pubsub.proto

package endpoint

import (
	context "context"
	proto "github.com/golang/protobuf/proto"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

// This is a compile-time assertion that a sufficiently up-to-date version
// of the legacy proto package is being used.
const _ = proto.ProtoPackageIsVersion4

type EndpointRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Req string `protobuf:"bytes,1,opt,name=req,proto3" json:"req,omitempty"`
}

func (x *EndpointRequest) Reset() {
	*x = EndpointRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_endpoint_pubsub_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *EndpointRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*EndpointRequest) ProtoMessage() {}

func (x *EndpointRequest) ProtoReflect() protoreflect.Message {
	mi := &file_endpoint_pubsub_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use EndpointRequest.ProtoReflect.Descriptor instead.
func (*EndpointRequest) Descriptor() ([]byte, []int) {
	return file_endpoint_pubsub_proto_rawDescGZIP(), []int{0}
}

func (x *EndpointRequest) GetReq() string {
	if x != nil {
		return x.Req
	}
	return ""
}

type EndpointResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Res string `protobuf:"bytes,1,opt,name=res,proto3" json:"res,omitempty"`
}

func (x *EndpointResponse) Reset() {
	*x = EndpointResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_endpoint_pubsub_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *EndpointResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*EndpointResponse) ProtoMessage() {}

func (x *EndpointResponse) ProtoReflect() protoreflect.Message {
	mi := &file_endpoint_pubsub_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use EndpointResponse.ProtoReflect.Descriptor instead.
func (*EndpointResponse) Descriptor() ([]byte, []int) {
	return file_endpoint_pubsub_proto_rawDescGZIP(), []int{1}
}

func (x *EndpointResponse) GetRes() string {
	if x != nil {
		return x.Res
	}
	return ""
}

var File_endpoint_pubsub_proto protoreflect.FileDescriptor

var file_endpoint_pubsub_proto_rawDesc = []byte{
	0x0a, 0x15, 0x65, 0x6e, 0x64, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x2f, 0x70, 0x75, 0x62, 0x73, 0x75,
	0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x08, 0x65, 0x6e, 0x64, 0x70, 0x6f, 0x69, 0x6e,
	0x74, 0x22, 0x23, 0x0a, 0x0f, 0x45, 0x6e, 0x64, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x52, 0x65, 0x71,
	0x75, 0x65, 0x73, 0x74, 0x12, 0x10, 0x0a, 0x03, 0x72, 0x65, 0x71, 0x18, 0x01, 0x20, 0x01, 0x28,
	0x09, 0x52, 0x03, 0x72, 0x65, 0x71, 0x22, 0x24, 0x0a, 0x10, 0x45, 0x6e, 0x64, 0x70, 0x6f, 0x69,
	0x6e, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x10, 0x0a, 0x03, 0x72, 0x65,
	0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x72, 0x65, 0x73, 0x32, 0x4c, 0x0a, 0x0b,
	0x47, 0x65, 0x74, 0x45, 0x6e, 0x64, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x12, 0x3d, 0x0a, 0x04, 0x53,
	0x65, 0x6e, 0x64, 0x12, 0x19, 0x2e, 0x65, 0x6e, 0x64, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x2e, 0x45,
	0x6e, 0x64, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x1a,
	0x2e, 0x65, 0x6e, 0x64, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x2e, 0x45, 0x6e, 0x64, 0x70, 0x6f, 0x69,
	0x6e, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x42, 0x45, 0x5a, 0x43, 0x67, 0x69,
	0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x6d, 0x65, 0x64, 0x69, 0x61, 0x2d, 0x73,
	0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x2d, 0x6d, 0x65, 0x73, 0x68, 0x2f, 0x61, 0x70,
	0x69, 0x2f, 0x76, 0x31, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x31, 0x2f, 0x72, 0x74, 0x73, 0x70, 0x2f,
	0x65, 0x6e, 0x64, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x3b, 0x65, 0x6e, 0x64, 0x70, 0x6f, 0x69, 0x6e,
	0x74, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_endpoint_pubsub_proto_rawDescOnce sync.Once
	file_endpoint_pubsub_proto_rawDescData = file_endpoint_pubsub_proto_rawDesc
)

func file_endpoint_pubsub_proto_rawDescGZIP() []byte {
	file_endpoint_pubsub_proto_rawDescOnce.Do(func() {
		file_endpoint_pubsub_proto_rawDescData = protoimpl.X.CompressGZIP(file_endpoint_pubsub_proto_rawDescData)
	})
	return file_endpoint_pubsub_proto_rawDescData
}

var file_endpoint_pubsub_proto_msgTypes = make([]protoimpl.MessageInfo, 2)
var file_endpoint_pubsub_proto_goTypes = []interface{}{
	(*EndpointRequest)(nil),  // 0: endpoint.EndpointRequest
	(*EndpointResponse)(nil), // 1: endpoint.EndpointResponse
}
var file_endpoint_pubsub_proto_depIdxs = []int32{
	0, // 0: endpoint.GetEndpoint.Send:input_type -> endpoint.EndpointRequest
	1, // 1: endpoint.GetEndpoint.Send:output_type -> endpoint.EndpointResponse
	1, // [1:2] is the sub-list for method output_type
	0, // [0:1] is the sub-list for method input_type
	0, // [0:0] is the sub-list for extension type_name
	0, // [0:0] is the sub-list for extension extendee
	0, // [0:0] is the sub-list for field type_name
}

func init() { file_endpoint_pubsub_proto_init() }
func file_endpoint_pubsub_proto_init() {
	if File_endpoint_pubsub_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_endpoint_pubsub_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*EndpointRequest); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_endpoint_pubsub_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*EndpointResponse); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_endpoint_pubsub_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   2,
			NumExtensions: 0,
			NumServices:   1,
		},
		GoTypes:           file_endpoint_pubsub_proto_goTypes,
		DependencyIndexes: file_endpoint_pubsub_proto_depIdxs,
		MessageInfos:      file_endpoint_pubsub_proto_msgTypes,
	}.Build()
	File_endpoint_pubsub_proto = out.File
	file_endpoint_pubsub_proto_rawDesc = nil
	file_endpoint_pubsub_proto_goTypes = nil
	file_endpoint_pubsub_proto_depIdxs = nil
}

// Reference imports to suppress errors if they are not otherwise used.
var _ context.Context
var _ grpc.ClientConnInterface

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
const _ = grpc.SupportPackageIsVersion6

// GetEndpointClient is the client API for GetEndpoint service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://godoc.org/google.golang.org/grpc#ClientConn.NewStream.
type GetEndpointClient interface {
	Send(ctx context.Context, in *EndpointRequest, opts ...grpc.CallOption) (*EndpointResponse, error)
}

type getEndpointClient struct {
	cc grpc.ClientConnInterface
}

func NewGetEndpointClient(cc grpc.ClientConnInterface) GetEndpointClient {
	return &getEndpointClient{cc}
}

func (c *getEndpointClient) Send(ctx context.Context, in *EndpointRequest, opts ...grpc.CallOption) (*EndpointResponse, error) {
	out := new(EndpointResponse)
	err := c.cc.Invoke(ctx, "/endpoint.GetEndpoint/Send", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// GetEndpointServer is the server API for GetEndpoint service.
type GetEndpointServer interface {
	Send(context.Context, *EndpointRequest) (*EndpointResponse, error)
}

// UnimplementedGetEndpointServer can be embedded to have forward compatible implementations.
type UnimplementedGetEndpointServer struct {
}

func (*UnimplementedGetEndpointServer) Send(context.Context, *EndpointRequest) (*EndpointResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Send not implemented")
}

func RegisterGetEndpointServer(s *grpc.Server, srv GetEndpointServer) {
	s.RegisterService(&_GetEndpoint_serviceDesc, srv)
}

func _GetEndpoint_Send_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(EndpointRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(GetEndpointServer).Send(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/endpoint.GetEndpoint/Send",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(GetEndpointServer).Send(ctx, req.(*EndpointRequest))
	}
	return interceptor(ctx, in, info, handler)
}

var _GetEndpoint_serviceDesc = grpc.ServiceDesc{
	ServiceName: "endpoint.GetEndpoint",
	HandlerType: (*GetEndpointServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "Send",
			Handler:    _GetEndpoint_Send_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "endpoint/pubsub.proto",
}
